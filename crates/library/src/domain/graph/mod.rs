mod display;
mod graph_error;
mod iterator;
mod schema;
pub(crate) mod semantic_validator;
pub(crate) mod structural_validator;

#[cfg(test)]
mod display_tests;
#[cfg(test)]
pub(crate) mod graph_asserter;
#[cfg(test)]
mod graph_tests;

use std::collections::{HashMap, HashSet};

use crate::{
    domain::{EdgeType, GateType, Position, graph::graph_error::GraphError},
    utils::{formatter, math},
    view::{GraphNodeView, NodeEdgeView},
};

#[derive(Debug, Clone)]
pub(self) struct NodeData {
    pub(self) gate: GateType,
    pub(self) angle: Option<f64>,
    pub(self) bit: Option<usize>,
}

impl PartialEq for NodeData {
    fn eq(&self, other: &Self) -> bool {
        self.gate == other.gate
            && math::are_option_floats_similar(self.angle, other.angle)
            && self.bit == other.bit
    }
}

impl Eq for NodeData {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(self) struct EdgeData {
    pub(self) edge_type: EdgeType,
    pub(self) other: Position,
}

/// Represents a quantum circuit as a hybrid of a directed graph and a 2D matrix.
///
/// This structured graph makes it easier to analyze and simplify circuits.
/// Position values of the form (row, column) are used to index the graph's nodes.
/// Each qubit is represented by a row in the grid.
/// Single-qubit gates occupy a single node, while multi-qubit gates use multiple related nodes (one for each qubit involved).
/// Empty spaces have no nodes, and for practical purposes, identity gates are also considered empty.
/// No duplicate edges (same origin, type and target) are allowed.
/// It's recommended to use the graph builder to build the graph with ease.
#[derive(Default, Debug, Clone)]
pub struct Graph {
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
    /// Edge insertion order is not taken into account.
    /// Floating point values are compared using a relative tolerance of 1e-5 and an absolute tolerance of 1e-8.
    /// This means that graph equality is non-transitive, but in practice this should be fine because floats are already non-deterministic across different hardware.
    fn eq(&self, other: &Self) -> bool {
        if self.nodes.len() != other.nodes.len() {
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
            let other_edges = match other.edges_out.get(start) {
                Some(edge) => edge,
                None => return false,
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
    /// Create an empty `Graph`.
    pub fn new() -> Self {
        Self::default()
    }

    /// The number of columns in the graph. Also known as the graph depth.
    pub fn width(&self) -> usize {
        self.nodes
            .keys()
            .map(|position| position.column())
            .max()
            .map(|column| column + 1)
            .unwrap_or(0)
    }

    /// The number of rows (qubits) in the graph.
    pub fn height(&self) -> usize {
        self.nodes
            .keys()
            .map(|position| position.row())
            .max()
            .map(|row| row + 1)
            .unwrap_or(0)
    }

    /// The number of classical bits in the graph.
    pub fn bits(&self) -> usize {
        self.nodes
            .values()
            .filter(|node| node.gate == GateType::Measure)
            .filter_map(|node| node.bit)
            .max()
            .map(|bit| bit + 1)
            .unwrap_or(0)
    }

    /// Check whether this graph is empty (has no gates) or not.
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// Get the total number of nodes in the graph.
    pub fn size(&self) -> usize {
        self.nodes.len()
    }

    /// Add a new node to the graph.
    ///
    /// Returns an error if a node already exists at the specified position.
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

        self.replace_node(gate, position, angle, bit);
        Ok(())
    }

    /// Replace the node at the specified position with the one provided.
    ///
    /// The node at the specified position is overwritten if present, or inserted if not.
    pub fn replace_node(
        &mut self,
        gate: GateType,
        position: Position,
        angle: Option<f64>,
        bit: Option<usize>,
    ) {
        self.nodes.insert(position, NodeData { gate, angle, bit });
    }

    /// Remove the node at the specified position and all its edges.
    ///
    /// If the node does not exist, nothing happens.
    /// If the removed node is horizontally between two nodes, these two nodes are joined.
    pub fn remove_node(&mut self, position: Position) {
        if !self.has_node_at(position) {
            return;
        }

        let previous = self.previous_in_row(position);
        let next = self.next_in_row(position);

        if let Some(out_edges) = self.edges_out.remove(&position) {
            for outgoing_edge in out_edges {
                if let Some(in_edges) = self.edges_in.get_mut(&outgoing_edge.other) {
                    in_edges.retain(|edge| {
                        !(edge.other == position && edge.edge_type == outgoing_edge.edge_type)
                    });
                }
            }
        }

        if let Some(in_edges) = self.edges_in.remove(&position) {
            for incoming_edge in in_edges {
                if let Some(out_edges) = self.edges_out.get_mut(&incoming_edge.other) {
                    out_edges.retain(|edge| {
                        !(edge.other == position && edge.edge_type == incoming_edge.edge_type)
                    });
                }
            }
        }

        self.nodes.remove(&position);

        if let (Some(left), Some(right)) = (previous, next) {
            self.add_edge_internal(EdgeType::Right, left, right);
        }
    }

    /// Move the node at the specified position to another position, removing the node at the destination and its edges.
    ///
    /// Using this with multi-qubit gates will most likely break the graph's semantic integrity.
    /// If no node exists at the start position, an error is returned.
    /// An error is also returned if the start and end positions are the same.
    pub fn move_node(&mut self, start: Position, end: Position) -> Result<(), GraphError> {
        if !self.has_node_at(start) {
            return Err(GraphError::NodeNotFound { position: start });
        }

        if start == end {
            return Err(GraphError::NullMove);
        }

        self.remove_node(end);

        let node = self.nodes.remove(&start).unwrap();
        let outgoing = self.edges_out.remove(&start).unwrap_or_default();
        let incoming = self.edges_in.remove(&start).unwrap_or_default();

        for outgoing_edge in &outgoing {
            if let Some(in_edges) = self.edges_in.get_mut(&outgoing_edge.other) {
                in_edges.retain(|edge| {
                    !(edge.other == start && edge.edge_type == outgoing_edge.edge_type)
                });
            }
        }

        for incoming_edge in &incoming {
            if let Some(out_edges) = self.edges_out.get_mut(&incoming_edge.other) {
                out_edges.retain(|edge| {
                    !(edge.other == start && edge.edge_type == incoming_edge.edge_type)
                });
            }
        }

        self.nodes.insert(end, node);

        for edge in outgoing {
            self.add_edge_internal(edge.edge_type, end, edge.other);
        }

        for edge in incoming {
            self.add_edge_internal(edge.edge_type, edge.other, end);
        }

        Ok(())
    }

    /// Check whether the graph has a node at the specified row and column.
    pub fn has_node_at(&self, position: Position) -> bool {
        self.nodes.contains_key(&position)
    }

    /// Check whether the graph has a non-identity node at the specified row and column.
    pub fn is_occupied(&self, position: Position) -> bool {
        self.nodes
            .get(&position)
            .map(|node| node.gate != GateType::ID)
            .unwrap_or(false)
    }

    /// Retrieve a read-only view of the node at the specified position.
    ///
    /// Returns None if no node exists at that position.
    /// Exposes node data in a view-friendly format, decoupling the internal representation from consumers.
    pub fn get_node(&self, position: Position) -> Option<GraphNodeView> {
        self.nodes
            .get(&position)
            .map(|node| GraphNodeView::new(node.gate, position, node.angle, node.bit))
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

        if edge_type.is_bidirectional() {
            if !self.has_edge(edge_type, end, start) {
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
    }

    fn has_edge(&self, edge_type: EdgeType, start: Position, end: Position) -> bool {
        self.edges_out
            .get(&start)
            .map(|edges| {
                edges
                    .iter()
                    .any(|edge| edge.edge_type == edge_type && edge.other == end)
            })
            .unwrap_or(false)
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
    pub fn clear(&mut self) {
        self.nodes.clear();
        self.edges_out.clear();
        self.edges_in.clear();
    }

    /// Remove all the edges from the graph.
    pub fn clear_edges(&mut self) {
        self.edges_out.clear();
        self.edges_in.clear();
    }

    /// Get the next node in the same row starting from the specified position, or `None` if there is none.
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
    pub fn get_node_and_edges(&self, position: Position) -> Option<NodeEdgeView> {
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
                let target = match self.get_node(edge.other) {
                    Some(node) => node,
                    None => continue,
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
                let source = match self.get_node(edge.other) {
                    Some(node) => node,
                    None => continue,
                };

                match edge.edge_type {
                    Right => left = Some(source),
                    Targets => controlled_by.push(source),
                    _ => {}
                }
            }
        }

        Some(NodeEdgeView::new(
            origin,
            left,
            right,
            targets,
            controlled_by,
            swaps_with,
            works_with,
        ))
    }

    /// Get an alternative string representation of the graph, as a 2D grid.
    pub fn display_grid(&self) -> String {
        if self.is_empty() {
            return "(empty)".to_owned();
        }

        let height = self.height();
        let width = self.width();

        let mut grid = vec![vec![".".to_string(); width]; height];

        for position in self.iter_positions_ordered_by_row() {
            if let Some(node) = self.get_node(position) {
                let mut label = node.r#type().to_string().to_ascii_uppercase();

                if let Some(angle) = node.angle() {
                    label.push('(');
                    label.push_str(&formatter::format_angle(angle));
                    label.push(')');
                }

                if let Some(bit) = node.bit() {
                    label.push('(');
                    label.push_str(&bit.to_string());
                    label.push(')');
                }

                grid[position.row()][position.column()] = label;
            }
        }

        let mut column_widths = vec![0; width];

        for column in 0..width {
            let max_length = (0..height)
                .map(|row| grid[row][column].len())
                .max()
                .unwrap_or(0);

            column_widths[column] = max_length;
        }

        let mut rows = Vec::new();

        for (row_index, row) in grid.iter().enumerate() {
            let formatted: Vec<String> = row
                .iter()
                .enumerate()
                .map(|(column, value)| format!("{:<width$}", value, width = column_widths[column]))
                .collect();

            let line = formatted.join("   ");
            let trimmed = line.trim_end();

            rows.push(format!("{row_index}: {}", trimmed));
        }

        rows.join("\n")
    }

    /// Validate the graph and confirm that it is internally consistent.
    ///
    /// This is only needed for graphs that are built manually.
    /// Building the graph using the `GraphBuilder` will never result in an invalid graph.
    // Note: The only exception to this is using the `put_*` methods from the builder.
    pub fn validate(&self) -> Result<(), GraphError> {
        structural_validator::validate(self)?;
        semantic_validator::validate(self)?;
        Ok(())
    }

    #[cfg(debug_assertions)]
    pub(super) fn validate_internal(&self) {
        structural_validator::validate(self).unwrap();
        semantic_validator::validate(self).unwrap();
    }

    pub(crate) fn remap_positions(&mut self, mut function: impl FnMut(Position) -> Position) {
        let mut new_nodes = HashMap::with_capacity(self.nodes.len());
        let mut new_edges_out = HashMap::new();
        let mut new_edges_in = HashMap::<Position, Vec<EdgeData>>::new();

        for (position, node) in self.nodes.drain() {
            let new_position = function(position);
            new_nodes.insert(new_position, node);
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
    }
}
