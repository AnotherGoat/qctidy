mod schema;
mod validator;

use std::{
    collections::{HashMap, HashSet},
    fmt,
};

use thiserror::Error;

use crate::{
    domain::{EdgeType, GateType, Position},
    utils::math,
    view::{GraphEdgeView, GraphNodeView, NodeEdgeView},
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

/// Error that can occur while working with a `QuantumGraph`.
#[derive(Debug, Error)]
pub enum GraphError {
    #[error("Node already exists at {position}")]
    NodeAlreadyExists { position: Position },
    #[error("Node at {position} does not exist")]
    NodeNotFound { position: Position },
    #[error("Start and end positions cannot be equal")]
    NullMove,
    #[error("An edge starts from a non-existent node at {from}")]
    DanglingStartNode { from: Position },
    #[error("An edge of type {edge_type} end points to a non-existent node at {from} to {to}")]
    DanglingEndNode {
        edge_type: EdgeType,
        from: Position,
        to: Position,
    },
    #[error(
        "An edge of type {edge_type} from {from} to {to} is missing its symmetrical counterpart"
    )]
    MissingReverseEdge {
        edge_type: EdgeType,
        from: Position,
        to: Position,
    },
    #[error("An edge of type {edge_type} from {from} to {to} exists more than once")]
    DuplicateEdge {
        edge_type: EdgeType,
        from: Position,
        to: Position,
    },
    #[error("Invalid edge {edge_type:?} from {from} to {to} for gate {gate:?}")]
    InvalidEdgeForGate {
        gate: GateType,
        edge_type: EdgeType,
        from: Position,
        to: Position,
    },
    #[error("Invalid angle for gate {gate:?} at {position}")]
    InvalidAngle { gate: GateType, position: Position },
    #[error("Invalid bit for gate {gate:?} at {position}")]
    InvalidBit { gate: GateType, position: Position },
    #[error("Invalid gate structure")]
    InvalidGateStructure,
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
pub struct QuantumGraph {
    pub(self) nodes: HashMap<Position, NodeData>,
    // Note: `edges_out` is the single source of truth for a graph's edges
    pub(self) edges_out: HashMap<Position, Vec<EdgeData>>,
    // Note: `edges_in` is only used for fast lookups and should always mirror `edges_out`
    pub(self) edges_in: HashMap<Position, Vec<EdgeData>>,
}

impl PartialEq for QuantumGraph {
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

        for (pos, node) in &self.nodes {
            match other.nodes.get(pos) {
                Some(other_node) if node == other_node => {}
                _ => return false,
            }
        }

        if self.edges_out.len() != other.edges_out.len() {
            return false;
        }

        for (start, edges) in &self.edges_out {
            let other_edges = match other.edges_out.get(start) {
                Some(e) => e,
                None => return false,
            };

            if edges.len() != other_edges.len() {
                return false;
            }

            let set_a: HashSet<_> = edges.iter().map(|e| (e.edge_type, e.other)).collect();

            let set_b: HashSet<_> = other_edges.iter().map(|e| (e.edge_type, e.other)).collect();

            if set_a != set_b {
                return false;
            }
        }

        true
    }
}

impl Eq for QuantumGraph {}

impl fmt::Display for QuantumGraph {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(formatter, "Nodes:")?;
        for node in self.iter_nodes_by_row() {
            writeln!(formatter, "{node}")?;
        }

        writeln!(formatter, "\nEdges:")?;

        for edge in self.iter_edges() {
            writeln!(formatter, "{edge}")?;
        }

        Ok(())
    }
}

impl QuantumGraph {
    /// Create an empty quantum graph.
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
        use EdgeType::{Left, Right};

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
            self.add_edge(Right, left, right)
                .expect("Both nodes should exist");
            self.add_edge(Left, right, left)
                .expect("Both nodes should exist");
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
    ///
    /// This method exposes node data in a view-friendly format, decoupling the internal representation from consumers.
    pub fn get_node_view(&self, position: Position) -> Option<GraphNodeView> {
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

        let out_edges = self.edges_out.entry(start).or_default();

        if out_edges
            .iter()
            .any(|edge| edge.edge_type == edge_type && edge.other == end)
        {
            return;
        }

        out_edges.push(EdgeData {
            edge_type,
            other: end,
        });

        self.edges_in.entry(end).or_default().push(EdgeData {
            edge_type,
            other: start,
        });
    }

    /// Remove an edge from the graph.
    ///
    /// If the edge does not exist, nothing happens.
    pub fn remove_edge(&mut self, edge_type: EdgeType, start: Position, end: Position) {
        if let Some(outgoing_edges) = self.edges_out.get_mut(&start) {
            outgoing_edges.retain(|edge| !(edge.edge_type == edge_type && edge.other == end));
        }

        if let Some(incoming_edges) = self.edges_in.get_mut(&end) {
            incoming_edges.retain(|edge| !(edge.edge_type == edge_type && edge.other == start));
        }
    }

    /// Add a new bidirectional edge to the graph, as a pair of unidirectional edges.
    pub fn add_bidirectional_edge(
        &mut self,
        edge_type: EdgeType,
        first: Position,
        second: Position,
    ) -> Result<(), GraphError> {
        self.add_edge(edge_type, first, second)?;
        self.add_edge(edge_type, second, first)
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

    /// Connect the left-right row neighbors of the node at the specified position.
    ///
    /// Returns an error if the node at the specified position does not exist.
    /// If there's an existing connection between the neighbors, it is removed and split into two.
    pub fn connect_row_neighbors(&mut self, position: Position) -> Result<(), GraphError> {
        use EdgeType::{Left, Right};

        if !self.has_node_at(position) {
            return Err(GraphError::NodeNotFound { position });
        }

        let previous_in_row = self.previous_in_row(position);
        let next_in_row = self.next_in_row(position);

        if let (Some(previous), Some(next)) = (previous_in_row, next_in_row) {
            self.remove_edge(Right, previous, next);
            self.remove_edge(Left, next, previous);
        }

        if let Some(previous) = previous_in_row {
            self.add_edge(Right, previous, position)?;
            self.add_edge(Left, position, previous)?;
        }

        if let Some(next) = next_in_row {
            self.add_edge(Right, position, next)?;
            self.add_edge(Left, next, position)?;
        }

        Ok(())
    }

    /// Iterate over the graph's positions, first row by row and then column by column.
    ///
    /// Positions without nodes or with identity nodes are skipped.
    pub fn iter_positions_by_row(&self) -> impl Iterator<Item = Position> + '_ {
        let height = self.height();
        let width = self.width();

        (0..height).flat_map(move |row| {
            (0..width).filter_map(move |column| {
                let position = Position::new(row, column);
                if self.has_node_at(position) {
                    Some(position)
                } else {
                    None
                }
            })
        })
    }

    /// Iterate over the graph's positions, first column by column and then row by row.
    ///
    /// Positions without nodes or with identity nodes are skipped.
    pub fn iter_positions_by_column(&self) -> impl Iterator<Item = Position> + '_ {
        let height = self.height();
        let width = self.width();

        (0..width).flat_map(move |column| {
            (0..height).filter_map(move |row| {
                let position = Position::new(row, column);
                if self.has_node_at(position) {
                    Some(position)
                } else {
                    None
                }
            })
        })
    }

    /// Iterate over all nodes in arbitrary order.
    ///
    /// The iteration order is not guaranteed and depends on the internal `HashMap`.
    /// Use `iter_nodes_by_row` or `iter_nodes_by_column` for deterministic traversal.
    pub fn iter_nodes(&self) -> impl Iterator<Item = GraphNodeView> + '_ {
        self.nodes
            .iter()
            .map(|(position, node)| GraphNodeView::new(node.gate, *position, node.angle, node.bit))
    }

    /// Retrieve all nodes in arbitrary order.
    ///
    /// Equivalent to collecting the output of `iter_nodes`.
    /// The order is not guaranteed. Use the ordered iterators if needed.
    pub fn nodes(&self) -> Vec<GraphNodeView> {
        self.iter_nodes().collect()
    }

    /// Iterate over the graph's nodes, first row by row and then column by column.
    ///
    /// This provides deterministic traversal based on a grid structure.
    /// Empty or identity nodes are skipped.
    pub fn iter_nodes_by_row(&self) -> impl Iterator<Item = GraphNodeView> + '_ {
        self.iter_positions_by_row()
            .filter_map(|position| self.get_node_view(position))
    }

    /// Iterate over the graph's nodes, first column by column and then row by row.
    ///
    /// This provides deterministic traversal based on a grid structure.
    /// Empty or identity nodes are skipped.
    pub fn iter_nodes_by_column(&self) -> impl Iterator<Item = GraphNodeView> + '_ {
        self.iter_positions_by_column()
            .filter_map(|position| self.get_node_view(position))
    }

    /// Iterate over the edges in the graph.
    pub fn iter_edges(&self) -> impl Iterator<Item = GraphEdgeView> + '_ {
        self.edges_out.iter().flat_map(move |(start, edges)| {
            edges.iter().filter_map(move |edge| {
                let start_view = self.get_node_view(*start)?;
                let end_view = self.get_node_view(edge.other)?;

                Some(GraphEdgeView::new(edge.edge_type, start_view, end_view))
            })
        })
    }

    /// Retrieve all the edges in the graph.
    ///
    /// Equivalent to collecting the output of `iter_edges`.
    /// The order of edges is not guaranteed.
    pub fn edges(&self) -> Vec<GraphEdgeView> {
        self.iter_edges().collect()
    }

    /// Iterate over the edges of the node at the specified position.
    pub fn iter_node_edges(&self, position: Position) -> impl Iterator<Item = GraphEdgeView> + '_ {
        self.edges_out
            .get(&position)
            .into_iter()
            .flat_map(move |edges| {
                edges.iter().filter_map(move |edge| {
                    let start = self.get_node_view(position)?;
                    let end = self.get_node_view(edge.other)?;

                    Some(GraphEdgeView::new(edge.edge_type, start, end))
                })
            })
    }

    /// Retrieve a node-edge view of the node at the specified position.
    pub fn node_edge_view(&self, position: Position) -> Option<NodeEdgeView> {
        use EdgeType::*;

        let origin = self.get_node_view(position)?;

        let mut left = None;
        let mut right = None;
        let mut swaps_with = None;

        let mut targets = Vec::new();
        let mut controlled_by = Vec::new();
        let mut works_with = Vec::new();

        if let Some(edges) = self.edges_out.get(&position) {
            for edge in edges {
                let target_node = match self.get_node_view(edge.other) {
                    Some(n) => n,
                    None => continue,
                };

                match edge.edge_type {
                    Left => left = Some(target_node),
                    Right => right = Some(target_node),
                    SwapsWith => swaps_with = Some(target_node),
                    Targets => targets.push(target_node),
                    WorksWith => works_with.push(target_node),
                    ControlledBy => {}
                }
            }
        }

        if let Some(edges) = self.edges_in.get(&position) {
            for edge in edges {
                if edge.edge_type == EdgeType::ControlledBy {
                    if let Some(node) = self.get_node_view(edge.other) {
                        controlled_by.push(node);
                    }
                }
            }
        }

        Some(NodeEdgeView::new(
            origin,
            left,
            right,
            swaps_with,
            targets,
            controlled_by,
            works_with,
        ))
    }

    /// Get an alternative string representation of the graph, as a 2D grid.
    pub fn draw_grid(&self) -> String {
        let height = self.height();
        let width = self.width();

        let mut grid = vec![vec![".".to_string(); width]; height];

        for position in self.iter_positions_by_row() {
            if let Some(node) = self.get_node_view(position) {
                grid[position.row()][position.column()] = node.r#type().to_string();
            }
        }

        let mut column_widths = vec![0; width];
        for column in 0..width {
            let max_len = (0..height)
                .map(|row| grid[row][column].len())
                .max()
                .unwrap_or(0);
            column_widths[column] = max_len;
        }

        let mut rows = Vec::new();

        for (row_idx, row) in grid.iter().enumerate() {
            let formatted: Vec<String> = row
                .iter()
                .enumerate()
                .map(|(column, val)| format!("{:<width$}", val, width = column_widths[column]))
                .collect();

            rows.push(format!("{row_idx}: {}", formatted.join("   ")));
        }

        rows.join("\n")
    }

    /// Validate the graph and confirm that it is internally consistent.
    ///
    /// This is only needed for graphs that are built manually.
    /// Building the graph using the `GraphBuilder` and using only the `push_*` methods will never result in an invalid graph.
    pub fn validate(&self) -> Result<(), GraphError> {
        self.check_dangling_nodes()?;
        self.check_edge_symmetry()?;
        self.check_duplicate_edges()?;
        validator::validate_graph_structure(self)?;
        Ok(())
    }

    #[cfg(debug_assertions)]
    pub(super) fn validate_internal(&self) {
        self.check_dangling_nodes().unwrap();
        self.check_edge_symmetry().unwrap();
        self.check_duplicate_edges().unwrap();
        validator::validate_graph_structure(self).unwrap();
    }

    fn check_dangling_nodes(&self) -> Result<(), GraphError> {
        for (start, edges) in &self.edges_out {
            if !self.nodes.contains_key(start) {
                return Err(GraphError::DanglingStartNode { from: *start });
            }

            for edge in edges {
                if !self.nodes.contains_key(&edge.other) {
                    return Err(GraphError::DanglingEndNode {
                        edge_type: edge.edge_type,
                        from: *start,
                        to: edge.other,
                    });
                }
            }
        }

        Ok(())
    }

    fn check_edge_symmetry(&self) -> Result<(), GraphError> {
        for (start, edges) in &self.edges_out {
            for edge in edges {
                let is_symmetric = self
                    .edges_in
                    .get(&edge.other)
                    .map(|other_edges| {
                        other_edges.iter().any(|other_edge| {
                            other_edge.other == *start && other_edge.edge_type == edge.edge_type
                        })
                    })
                    .unwrap_or(false);

                if !is_symmetric {
                    return Err(GraphError::MissingReverseEdge {
                        from: *start,
                        to: edge.other,
                        edge_type: edge.edge_type,
                    });
                }
            }
        }

        Ok(())
    }

    fn check_duplicate_edges(&self) -> Result<(), GraphError> {
        for (start, edges) in &self.edges_out {
            let mut seen = HashSet::new();

            for edge in edges {
                if !seen.insert((edge.edge_type, edge.other)) {
                    return Err(GraphError::DuplicateEdge {
                        from: *start,
                        to: edge.other,
                        edge_type: edge.edge_type,
                    });
                }
            }
        }

        Ok(())
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
