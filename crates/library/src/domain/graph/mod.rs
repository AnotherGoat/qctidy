pub(crate) mod edge_type;
pub(crate) mod gate_type;
pub(crate) mod graph_builder;
pub(crate) mod graph_error;
mod iterator;
pub(crate) mod position;
mod schema;
mod semantic_validator;
mod structural_validator;

use crate::domain::projection::{ContextualNodeView, NodeView};

#[cfg(test)]
mod gate_type_tests;
#[cfg(test)]
pub(crate) mod graph_asserter;
#[cfg(test)]
mod graph_builder_tests;
#[cfg(test)]
mod graph_tests;
#[cfg(test)]
mod iterator_tests;

use std::collections::{HashMap, HashSet};

use crate::{
    EdgeType, GateType, Position,
    domain::{graph::graph_error::GraphError, math},
};

#[derive(Debug, Clone)]
#[must_use]
struct NodeData {
    pub(self) gate: GateType,
    pub(self) angle: Option<f64>,
    pub(self) bit: Option<usize>,
}

impl PartialEq for NodeData {
    fn eq(&self, other: &Self) -> bool {
        self.gate == other.gate
            && math::are_option_floats_equal(self.angle, other.angle)
            && self.bit == other.bit
    }
}

impl Eq for NodeData {}

#[derive(Debug, Clone, PartialEq, Eq)]
#[must_use]
struct EdgeData {
    pub(self) edge_type: EdgeType,
    pub(self) other: Position,
}

/// Represents a quantum circuit as a hybrid of a directed graph and a 2D matrix.
///
/// This structured graph makes it easier to analyze and simplify circuits.
/// Position values of the form (row, column) are used to index the graph's nodes.
/// Each qubit is represented by a row in the grid.
/// Single-qubit gates occupy a single node, while multi-qubit gates use multiple related nodes (one for each qubit involved).
/// Empty spaces have no nodes.
/// No duplicate edges (same origin, type and target) are allowed.
/// It's recommended to use the graph builder to build the graph with ease.
#[derive(Default, Debug, Clone)]
#[must_use]
pub struct Graph {
    qubit_count: usize,
    time_step_count: usize,
    row_node_counts: Vec<usize>,
    column_node_counts: Vec<usize>,
    bit_count: usize,
    pub(self) nodes: HashMap<Position, NodeData>,
    // Note: `edges_out` is the single source of truth for a graph's edges
    pub(self) edges_out: HashMap<Position, Vec<EdgeData>>,
    // Note: `edges_in` is only used for fast lookups and should always mirror `edges_out`
    pub(self) edges_in: HashMap<Position, Vec<EdgeData>>,
}

impl PartialEq for Graph {
    /// Check whether two graphs are equal.
    ///
    /// Node positions and data must match, as well as edges.
    /// Edge insertion order and graph width are not taken into account.
    /// Floating point values are compared using a relative tolerance of 1e-5 and an absolute tolerance of 1e-8.
    /// This means that graph equality is non-transitive, but in practice this should be fine because floats are already non-deterministic across different hardware.
    fn eq(&self, other: &Self) -> bool {
        if self.qubit_count != other.qubit_count || self.nodes.len() != other.nodes.len() {
            return false;
        }

        for (position, node) in &self.nodes {
            match other.nodes.get(position) {
                Some(other_node) if node == other_node => {}
                _ => return false,
            }
        }

        if self.edges_out.len() != other.edges_out.len() {
            return false;
        }

        for (start, edges) in &self.edges_out {
            let Some(other_edges) = other.edges_out.get(start) else {
                return false;
            };

            if edges.len() != other_edges.len() {
                return false;
            }

            let set: HashSet<_> = edges
                .iter()
                .map(|edge| (edge.edge_type, edge.other))
                .collect();
            let other_set: HashSet<_> = other_edges
                .iter()
                .map(|edge| (edge.edge_type, edge.other))
                .collect();

            if set != other_set {
                return false;
            }
        }

        true
    }
}

impl Eq for Graph {}

impl Graph {
    /// Create an empty `Graph` of the specified initial qubit count.
    pub fn new(initial_qubit_count: usize) -> Self {
        Self {
            qubit_count: initial_qubit_count,
            row_node_counts: vec![0; initial_qubit_count],
            ..Default::default()
        }
    }

    /// The number of columns (time steps) in the graph. Also known as the graph depth.
    ///
    /// O(1).
    #[must_use]
    pub const fn width(&self) -> usize {
        self.time_step_count
    }

    fn ensure_column_capacity(&mut self, column: usize) {
        if column >= self.column_node_counts.len() {
            self.column_node_counts.resize(column + 1, 0);
        }
    }

    /// The number of rows (qubits) in the graph.
    ///
    /// O(1).
    #[must_use]
    pub const fn height(&self) -> usize {
        self.qubit_count
    }

    fn ensure_row_capacity(&mut self, row: usize) {
        if row >= self.row_node_counts.len() {
            self.row_node_counts.resize(row + 1, 0);
        }
    }

    /// The number of classical bits in the graph.
    ///
    /// O(1).
    #[must_use]
    pub const fn bits(&self) -> usize {
        self.bit_count
    }

    /// Check whether this graph is empty (has no gates) or not.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// Get the total number of nodes in the graph.
    #[must_use]
    pub fn size(&self) -> usize {
        self.nodes.len()
    }

    /// Add a new node to the graph.
    ///
    /// Returns an error if a node already exists at the specified position.
    /// Increases the graph height if necessary.
    pub fn add_node(
        &mut self,
        gate: GateType,
        position: Position,
        angle: Option<f64>,
        bit: Option<usize>,
    ) -> Result<(), GraphError> {
        if self.has_node_at(position) {
            return Err(GraphError::NodeAlreadyExists { position });
        }

        let node = NodeData { gate, angle, bit };
        self.insert_new_node(position, node);
        Ok(())
    }

    fn insert_new_node(&mut self, position: Position, node: NodeData) {
        self.ensure_row_capacity(position.row());
        self.ensure_column_capacity(position.column());

        self.row_node_counts[position.row()] += 1;
        self.column_node_counts[position.column()] += 1;

        self.qubit_count = self.row_node_counts.len();
        self.time_step_count = self.time_step_count.max(position.column() + 1);

        if let Some(bit) = node.bit {
            self.bit_count = self.bit_count.max(bit + 1);
        }

        self.nodes.insert(position, node);
    }

    /// Replace the node at the specified position with the one provided.
    ///
    /// The node at the specified position is overwritten if present, or inserted if not.
    /// Increases the graph height if necessary (when the node is inserted).
    pub fn replace_node(
        &mut self,
        gate: GateType,
        position: Position,
        angle: Option<f64>,
        bit: Option<usize>,
    ) {
        let node = NodeData { gate, angle, bit };

        if self.has_node_at(position) {
            self.replace_existing_node(position, &node);
            return;
        }

        self.insert_new_node(position, node);
    }

    fn replace_existing_node(&mut self, position: Position, node: &NodeData) {
        let previous = self.nodes.insert(position, node.clone());

        if let Some(bit) = node.bit {
            self.bit_count = self.bit_count.max(bit + 1);
        }

        if let Some(previous_node) = previous
            && let Some(bit) = previous_node.bit
            && previous_node.gate == GateType::Measure
            && bit + 1 == self.bit_count
        {
            self.update_bit_count();
        }
    }

    /// Remove the node at the specified position and all its edges.
    ///
    /// If the node does not exist, nothing happens.
    /// If the removed node is horizontally between two nodes, these two nodes are joined.
    /// Note that removing nodes does not affect the graph height.
    pub fn remove_node(&mut self, position: Position) {
        let previous = self.previous_in_row(position);
        let next = self.next_in_row(position);

        if self.detach_node(position).is_none() {
            return;
        }

        if let (Some(left), Some(right)) = (previous, next) {
            self.add_edge_internal(EdgeType::Right, left, right);
        }
    }

    fn detach_node(
        &mut self,
        position: Position,
    ) -> Option<(NodeData, Vec<EdgeData>, Vec<EdgeData>)> {
        let node = self.nodes.remove(&position)?;

        self.row_node_counts[position.row()] -= 1;
        self.column_node_counts[position.column()] -= 1;

        let outgoing = self.edges_out.remove(&position).unwrap_or_default();
        let incoming = self.edges_in.remove(&position).unwrap_or_default();

        for outgoing_edge in &outgoing {
            if let Some(in_edges) = self.edges_in.get_mut(&outgoing_edge.other) {
                in_edges.retain(|edge| {
                    !(edge.other == position && edge.edge_type == outgoing_edge.edge_type)
                });
            }
        }

        for incoming_edge in &incoming {
            if let Some(out_edges) = self.edges_out.get_mut(&incoming_edge.other) {
                out_edges.retain(|edge| {
                    !(edge.other == position && edge.edge_type == incoming_edge.edge_type)
                });
            }
        }

        if position.column() + 1 == self.time_step_count
            && self.column_node_counts[position.column()] == 0
        {
            self.update_time_step_count();
        }

        if let Some(bit) = node.bit
            && node.gate == GateType::Measure
            && bit + 1 == self.bit_count
        {
            self.update_bit_count();
        }

        Some((node, outgoing, incoming))
    }

    fn update_time_step_count(&mut self) {
        while self.time_step_count > 0 {
            let last_column = self.time_step_count - 1;

            let occupied = self
                .column_node_counts
                .get(last_column)
                .is_some_and(|count| *count > 0);

            if occupied {
                break;
            }

            self.time_step_count -= 1;
        }
    }

    fn update_bit_count(&mut self) {
        self.bit_count = self
            .nodes
            .values()
            .filter(|node| node.gate == GateType::Measure)
            .filter_map(|node| node.bit)
            .max()
            .map_or(0, |bit| bit + 1);
    }

    /// Move the node at the specified position to another position, removing the node at the destination and its edges.
    ///
    /// Using this with multi-qubit gates will most likely break the graph's semantic integrity.
    /// If no node exists at the start position, an error is returned.
    /// An error is also returned if the start and end positions are the same.
    /// Increases the graph height if necessary.
    pub fn move_node(&mut self, start: Position, end: Position) -> Result<(), GraphError> {
        if !self.has_node_at(start) {
            return Err(GraphError::NodeNotFound { position: start });
        }

        if start == end {
            return Err(GraphError::NullMove);
        }

        self.remove_node(end);

        let (node, outgoing, incoming) = self
            .detach_node(start)
            .expect("Node at start position should exist");

        self.insert_new_node(end, node);

        for edge in outgoing {
            self.add_edge_internal(edge.edge_type, end, edge.other);
        }

        for edge in incoming {
            self.add_edge_internal(edge.edge_type, edge.other, end);
        }

        Ok(())
    }

    /// Check whether the graph has a node at the specified row and column.
    #[must_use]
    pub fn has_node_at(&self, position: Position) -> bool {
        self.nodes.contains_key(&position)
    }

    /// Check whether the graph has a node at the specified row and column.
    #[must_use]
    pub fn is_occupied(&self, position: Position) -> bool {
        self.nodes.contains_key(&position)
    }

    /// Retrieve a read-only projection of the node at the specified position.
    ///
    /// Returns None if no node exists at that position.
    /// Exposes node data in a view-friendly format, decoupling the internal representation from consumers.
    #[must_use]
    pub fn get_node(&self, position: Position) -> Option<NodeView> {
        self.nodes
            .get(&position)
            .map(|node| NodeView::new(node.gate, position, node.angle, node.bit))
    }

    /// Add a new edge to the graph.
    ///
    /// Returns an error if the start or end nodes do not exist.
    /// If the edge already exists, nothing happens.
    pub fn add_edge(
        &mut self,
        edge_type: EdgeType,
        start: Position,
        end: Position,
    ) -> Result<(), GraphError> {
        if !self.has_node_at(start) {
            return Err(GraphError::NodeNotFound { position: start });
        }

        if !self.has_node_at(end) {
            return Err(GraphError::NodeNotFound { position: end });
        }

        self.add_edge_internal(edge_type, start, end);

        Ok(())
    }

    fn add_edge_internal(&mut self, edge_type: EdgeType, start: Position, end: Position) {
        debug_assert!(self.has_node_at(start));
        debug_assert!(self.has_node_at(end));

        if self.has_edge(edge_type, start, end) {
            return;
        }

        self.edges_out.entry(start).or_default().push(EdgeData {
            edge_type,
            other: end,
        });

        self.edges_in.entry(end).or_default().push(EdgeData {
            edge_type,
            other: start,
        });

        if edge_type.is_bidirectional() && !self.has_edge(edge_type, end, start) {
            self.edges_out.entry(end).or_default().push(EdgeData {
                edge_type,
                other: start,
            });

            self.edges_in.entry(start).or_default().push(EdgeData {
                edge_type,
                other: end,
            });
        }
    }

    fn has_edge(&self, edge_type: EdgeType, start: Position, end: Position) -> bool {
        self.edges_out.get(&start).is_some_and(|edges| {
            edges
                .iter()
                .any(|edge| edge.edge_type == edge_type && edge.other == end)
        })
    }

    /// Remove an edge from the graph.
    ///
    /// If the edge does not exist, nothing happens.
    pub fn remove_edge(&mut self, edge_type: EdgeType, start: Position, end: Position) {
        self.remove_edge_internal(edge_type, start, end);
    }

    fn remove_edge_internal(&mut self, edge_type: EdgeType, start: Position, end: Position) {
        self.remove_single_edge(edge_type, start, end);

        if edge_type.is_bidirectional() {
            self.remove_single_edge(edge_type, end, start);
        }
    }

    fn remove_single_edge(&mut self, edge_type: EdgeType, start: Position, end: Position) {
        if let Some(outgoing_edges) = self.edges_out.get_mut(&start) {
            outgoing_edges.retain(|edge| !(edge.edge_type == edge_type && edge.other == end));
        }

        if let Some(incoming_edges) = self.edges_in.get_mut(&end) {
            incoming_edges.retain(|edge| !(edge.edge_type == edge_type && edge.other == start));
        }
    }

    /// Remove all the nodes and edges from the graph.
    ///
    /// Resets qubit count and bit count back to 0.
    pub fn clear(&mut self) {
        self.nodes.clear();
        self.edges_out.clear();
        self.edges_in.clear();

        self.qubit_count = 0;
        self.time_step_count = 0;
        self.bit_count = 0;

        self.row_node_counts.clear();
        self.column_node_counts.clear();
    }

    /// Remove all the edges from the graph.
    pub fn clear_edges(&mut self) {
        self.edges_out.clear();
        self.edges_in.clear();
    }

    /// Get the next node in the same row starting from the specified position, or `None` if there is none.
    #[must_use]
    pub fn next_in_row(&self, position: Position) -> Option<Position> {
        let row = position.row();
        let column = position.column();

        self.nodes
            .keys()
            .filter(|node_position| node_position.row() == row && node_position.column() > column)
            .min_by_key(|node_position| node_position.column())
            .copied()
    }

    /// Get the previous node in the same row starting from the specified position, or `None` if there is none.
    #[must_use]
    pub fn previous_in_row(&self, position: Position) -> Option<Position> {
        let row = position.row();
        let column = position.column();

        self.nodes
            .keys()
            .filter(|node_position| node_position.row() == row && node_position.column() < column)
            .max_by_key(|node_position| node_position.column())
            .copied()
    }

    /// Connect the positional row neighbors of the node at the specified position.
    ///
    /// Returns an error if the node at the specified position does not exist.
    /// If there's an existing connection between the neighbors, it is removed and split into two.
    pub fn connect_row_neighbors(&mut self, position: Position) -> Result<(), GraphError> {
        if !self.has_node_at(position) {
            return Err(GraphError::NodeNotFound { position });
        }

        let previous_in_row = self.previous_in_row(position);
        let next_in_row = self.next_in_row(position);

        if let (Some(previous), Some(next)) = (previous_in_row, next_in_row) {
            self.remove_edge_internal(EdgeType::Right, previous, next);
        }

        if let Some(previous) = previous_in_row {
            self.add_edge_internal(EdgeType::Right, previous, position);
        }

        if let Some(next) = next_in_row {
            self.add_edge_internal(EdgeType::Right, position, next);
        }

        Ok(())
    }

    /// Retrieve a node-edge view of the node at the specified position.
    ///
    /// Returns None if no node exists at that position.
    /// Exposes node surrounding data in a view-friendly format, decoupling the internal representation from consumers.
    #[must_use]
    pub fn get_contextual_view(&self, position: Position) -> Option<ContextualNodeView> {
        use EdgeType::*;

        let origin = self.get_node(position)?;

        let mut left = None;
        let mut right = None;

        let mut targets = Vec::new();
        let mut controlled_by = Vec::new();

        let mut swaps_with = None;
        let mut works_with = Vec::new();

        if let Some(edges) = self.edges_out.get(&position) {
            for edge in edges {
                let Some(target) = self.get_node(edge.other) else {
                    continue;
                };

                match edge.edge_type {
                    Right => right = Some(target),
                    Targets => targets.push(target),
                    SwapsWith => swaps_with = Some(target),
                    WorksWith => works_with.push(target),
                }
            }
        }

        if let Some(edges) = self.edges_in.get(&position) {
            for edge in edges {
                let Some(source) = self.get_node(edge.other) else {
                    continue;
                };

                match edge.edge_type {
                    Right => left = Some(source),
                    Targets => controlled_by.push(source),
                    _ => {}
                }
            }
        }

        Some(ContextualNodeView::new(
            origin,
            left,
            right,
            targets,
            controlled_by,
            swaps_with,
            works_with,
        ))
    }

    /// Validate the graph and confirm that it is internally consistent.
    ///
    /// This is only needed for graphs that are built manually.
    /// Building the graph using the `GraphBuilder` will always result in an valid graph, so calling this method is unnecessary for these cases.
    // Note: The only exception to this is using the `put_*` methods from the builder.
    pub fn validate(&self) -> Result<(), GraphError> {
        structural_validator::validate(self)?;
        semantic_validator::validate(self)?;
        Ok(())
    }

    #[cfg(debug_assertions)]
    #[expect(clippy::unwrap_used)]
    pub(crate) fn validate_internal(&self) {
        structural_validator::validate(self).unwrap();
        semantic_validator::validate(self).unwrap();
    }

    pub(crate) fn remap_positions(&mut self, mut function: impl FnMut(Position) -> Position) {
        self.qubit_count = 0;
        self.time_step_count = 0;
        self.bit_count = 0;

        self.row_node_counts.clear();
        self.column_node_counts.clear();

        let mut new_nodes = HashMap::with_capacity(self.nodes.len());
        let mut new_edges_out = HashMap::new();
        let mut new_edges_in = HashMap::<Position, Vec<EdgeData>>::new();

        for (position, node) in self.nodes.drain() {
            let new_position = function(position);
            let inserted = new_nodes.insert(new_position, node);

            debug_assert!(
                inserted.is_none(),
                "Position remap produced duplicate node positions"
            );
        }

        for (start, edges) in self.edges_out.drain() {
            let new_start = function(start);
            let new_edges = edges
                .into_iter()
                .map(|edge| EdgeData {
                    edge_type: edge.edge_type,
                    other: function(edge.other),
                })
                .collect::<Vec<_>>();

            new_edges_out.insert(new_start, new_edges);
        }

        for (start, edges) in &new_edges_out {
            for edge in edges {
                new_edges_in.entry(edge.other).or_default().push(EdgeData {
                    edge_type: edge.edge_type,
                    other: *start,
                });
            }
        }

        self.nodes = new_nodes;
        self.edges_out = new_edges_out;
        self.edges_in = new_edges_in;

        let node_data = self
            .nodes
            .iter()
            .map(|(position, node)| (*position, node.bit))
            .collect::<Vec<_>>();

        for (position, bit) in node_data {
            self.ensure_row_capacity(position.row());
            self.ensure_column_capacity(position.column());

            self.row_node_counts[position.row()] += 1;
            self.column_node_counts[position.column()] += 1;

            self.time_step_count = self.time_step_count.max(position.column() + 1);

            if let Some(bit) = bit {
                self.bit_count = self.bit_count.max(bit + 1);
            }
        }

        self.qubit_count = self.row_node_counts.len();
    }
}
