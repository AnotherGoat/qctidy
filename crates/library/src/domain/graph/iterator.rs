use std::collections::HashSet;

use crate::{EdgeType, EdgeView, Graph, NodeView, Position, domain::graph::EdgeData};

impl Graph {
    /// Iterate over the graph's positions that have nodes, ordered by qubit and then by time step.
    ///
    /// Positions without nodes are skipped.
    ///
    /// Complexity: O(n).
    pub fn iter_positions_ordered_by_qubit(&self) -> impl Iterator<Item = Position> + '_ {
        self.qubit_time_steps
            .iter()
            .enumerate()
            .flat_map(|(qubit, time_steps)| {
                time_steps
                    .iter()
                    .map(move |&time_step| Position::new(qubit, time_step))
            })
    }

    /// Iterate over the graph's positions that have nodes, ordered by time step and then by qubit.
    ///
    /// Positions without nodes are skipped.
    ///
    /// Complexity: O(n).
    pub fn iter_positions_ordered_by_time_step(&self) -> impl Iterator<Item = Position> + '_ {
        self.time_step_qubits
            .iter()
            .enumerate()
            .flat_map(|(time_step, qubits)| {
                qubits
                    .iter()
                    .map(move |&qubit| Position::new(qubit, time_step))
            })
    }

    /// Iterate over all nodes in arbitrary order.
    ///
    /// The iteration order is not guaranteed and depends on the internal `HashMap`.
    /// Use `iter_nodes_ordered_by_qubit` or `iter_nodes_ordered_by_time_step` for deterministic traversal.
    ///
    /// Complexity: O(n).
    pub fn iter_nodes(&self) -> impl Iterator<Item = NodeView> + '_ {
        self.nodes.iter().map(|(position, node)| {
            NodeView::new(
                node.gate,
                *position,
                node.theta,
                node.phi,
                node.lambda,
                node.bit,
            )
        })
    }

    /// Iterate over the graph's nodes, ordered by qubit and then by time step.
    ///
    /// This provides deterministic traversal based on a grid structure.
    /// Positions without nodes are skipped.
    ///
    /// Complexity: O(n).
    pub fn iter_nodes_ordered_by_qubit(&self) -> impl Iterator<Item = NodeView> + '_ {
        self.iter_positions_ordered_by_qubit()
            .filter_map(|position| self.get_node(position))
    }

    /// Iterate over the graph's nodes, ordered by time step and then by qubit.
    ///
    /// This provides deterministic traversal based on a grid structure.
    /// Positions without nodes are skipped.
    ///
    /// Complexity: O(n).
    pub fn iter_nodes_ordered_by_time_step(&self) -> impl Iterator<Item = NodeView> + '_ {
        self.iter_positions_ordered_by_time_step()
            .filter_map(|position| self.get_node(position))
    }

    pub(crate) fn iter_edges_by_time_step(&self) -> impl Iterator<Item = EdgeView> + '_ {
        self.iter_positions_ordered_by_time_step()
            .flat_map(move |position| {
                self.sorted_node_edges(position, |edge| {
                    (edge.edge_type, edge.other.time_step(), edge.other.qubit())
                })
            })
    }

    fn sorted_node_edges(
        &self,
        position: Position,
        key: impl Fn(&EdgeData) -> (EdgeType, usize, usize),
    ) -> Vec<EdgeView> {
        let Some(start) = self.get_node(position) else {
            return Vec::new();
        };

        let mut edges: Vec<_> = self
            .edges_out
            .get(&position)
            .into_iter()
            .flatten()
            .collect();

        edges.sort_by_key(|edge| key(edge));

        edges
            .into_iter()
            .filter_map(|edge| {
                let end = self.get_node(edge.other)?;
                Some(EdgeView::new(edge.edge_type, start, end))
            })
            .collect()
    }

    /// Iterate over edges in the graph in arbitrary order.
    ///
    /// Bidirectional edges like `SwapsWith` and `WorksWith` will be duplicated.
    /// To get deduplicated edges, use `iter_edges_unique`.
    ///
    /// Complexity: O(n + edges).
    pub fn iter_edges(&self) -> impl Iterator<Item = EdgeView> + '_ {
        self.nodes
            .keys()
            .flat_map(move |position| self.iter_edges_from(*position))
    }

    /// Iterate over edges in the graph in arbitrary order.
    ///
    /// Bidirectional edges like `SwapsWith` and `WorksWith` are deduplicated.
    ///
    /// Complexity: O(n + edges).
    pub fn iter_edges_unique(&self) -> impl Iterator<Item = EdgeView> + '_ {
        Self::deduplicate_edges(self.iter_edges())
    }

    /// Iterate over the edges around the node at the specified position, in arbitrary order.
    ///
    /// Bidirectional edges like `SwapsWith` and `WorksWith` will be duplicated.
    /// To get deduplicated node edges, use `iter_edges_from_unique`.
    ///
    /// Complexity: O(degree).
    pub fn iter_edges_from(&self, position: Position) -> impl Iterator<Item = EdgeView> + '_ {
        let mut edges = Vec::new();

        if let Some(outgoing) = self.edges_out.get(&position) {
            for edge in outgoing {
                if let (Some(start), Some(end)) =
                    (self.get_node(position), self.get_node(edge.other))
                {
                    edges.push(EdgeView::new(edge.edge_type, start, end));
                }
            }
        }

        edges.into_iter()
    }

    /// Iterate over the edges around the node at the specified position, in arbitrary order.
    ///
    /// Bidirectional edges like `SwapsWith` and `WorksWith` are deduplicated.
    ///
    /// Complexity: O(degree).
    pub fn iter_edges_from_unique(
        &self,
        position: Position,
    ) -> impl Iterator<Item = EdgeView> + '_ {
        Self::deduplicate_edges(self.iter_edges_from(position))
    }

    fn deduplicate_edges(
        iterator: impl Iterator<Item = EdgeView>,
    ) -> impl Iterator<Item = EdgeView> {
        // Complexity: O(k), where k is the number of yielded edges.
        let mut seen = HashSet::new();

        iterator.filter(move |edge| {
            let start = edge.start().position();
            let end = edge.end().position();
            let r#type = edge.r#type();

            let key = if r#type.is_bidirectional() && end < start {
                (r#type, end, start)
            } else {
                (r#type, start, end)
            };

            seen.insert(key)
        })
    }

    /// Iterate over the semantic edges leaving the node at the specified position.
    ///
    /// Complexity: O(degree).
    pub fn iter_semantic_edges_out_from(
        &self,
        position: Position,
    ) -> impl Iterator<Item = EdgeView> + '_ {
        self.iter_edges_from_unique(position)
            .filter(|edge| edge.r#type().is_semantic())
    }

    /// Iterate over the semantic neighbors of the node at the specified position.
    ///
    /// Complexity: O(degree).
    pub fn iter_semantic_neighbors_from(
        &self,
        start: Position,
    ) -> impl Iterator<Item = Position> + '_ {
        let mut seen = HashSet::new();

        let outgoing = self
            .edges_out
            .get(&start)
            .into_iter()
            .flatten()
            .filter(move |edge| edge.edge_type.is_semantic())
            .map(|edge| edge.other);

        let incoming = self
            .edges_in
            .get(&start)
            .into_iter()
            .flatten()
            .filter(move |edge| edge.edge_type.is_semantic())
            .map(|edge| edge.other);

        outgoing
            .chain(incoming)
            .filter(move |position| seen.insert(*position))
    }
}
