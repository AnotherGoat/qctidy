use std::collections::HashSet;

use crate::{EdgeType, EdgeView, Graph, NodeView, Position, domain::graph::EdgeData};

impl Graph {
    /// Iterate over the graph's positions that have nodes, first row and then by column.
    ///
    /// Positions without nodes or with identity nodes are skipped.
    pub fn iter_positions_ordered_by_row(&self) -> impl Iterator<Item = Position> + '_ {
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

    /// Iterate over the graph's positions that have nodes, first column and then by row.
    ///
    /// Positions without nodes or with identity nodes are skipped.
    pub fn iter_positions_ordered_by_column(&self) -> impl Iterator<Item = Position> + '_ {
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
    pub fn iter_nodes(&self) -> impl Iterator<Item = NodeView> + '_ {
        self.nodes
            .iter()
            .map(|(position, node)| NodeView::new(node.gate, *position, node.angle, node.bit))
    }

    /// Iterate over the graph's nodes, first row by row and then column by column.
    ///
    /// This provides deterministic traversal based on a grid structure.
    /// Empty or identity nodes are skipped.
    pub fn iter_nodes_ordered_by_row(&self) -> impl Iterator<Item = NodeView> + '_ {
        self.iter_positions_ordered_by_row()
            .filter_map(|position| self.get_node(position))
    }

    /// Iterate over the graph's nodes, first column by column and then row by row.
    ///
    /// This provides deterministic traversal based on a grid structure.
    /// Empty or identity nodes are skipped.
    pub fn iter_nodes_ordered_by_column(&self) -> impl Iterator<Item = NodeView> + '_ {
        self.iter_positions_ordered_by_column()
            .filter_map(|position| self.get_node(position))
    }

    pub(crate) fn iter_edges_by_row(&self) -> impl Iterator<Item = EdgeView> + '_ {
        self.iter_positions_ordered_by_row()
            .flat_map(move |position| {
                self.sorted_node_edges(position, |edge| {
                    (edge.edge_type, edge.other.row(), edge.other.column())
                })
            })
    }

    pub(crate) fn iter_edges_by_column(&self) -> impl Iterator<Item = EdgeView> + '_ {
        self.iter_positions_ordered_by_column()
            .flat_map(move |position| {
                self.sorted_node_edges(position, |edge| {
                    (edge.edge_type, edge.other.column(), edge.other.row())
                })
            })
    }

    fn sorted_node_edges(
        &self,
        position: Position,
        key: impl Fn(&EdgeData) -> (EdgeType, usize, usize),
    ) -> Vec<EdgeView> {
        let mut edges: Vec<_> = self
            .edges_out
            .get(&position)
            .into_iter()
            .flatten()
            .collect();

        edges.sort_by_key(|edge| key(edge));

        edges
            .into_iter()
            .filter_map(move |edge| {
                let start = self.get_node(position)?;
                let end = self.get_node(edge.other)?;
                Some(EdgeView::new(edge.edge_type, start, end))
            })
            .collect()
    }

    /// Iterate over edges in the graph in arbitrary order.
    ///
    /// Bidirectional edges like `SwapsWith` and `WorksWith` will be duplicated.
    /// To get deduplicated edges, use `iter_edges_unique`.
    pub fn iter_edges(&self) -> impl Iterator<Item = EdgeView> + '_ {
        self.nodes
            .keys()
            .flat_map(move |position| self.iter_edges_from(*position))
    }

    /// Iterate over edges in the graph in arbitrary order.
    ///
    /// Bidirectional edges like `SwapsWith` and `WorksWith` are deduplicated.
    pub fn iter_edges_unique(&self) -> impl Iterator<Item = EdgeView> + '_ {
        Self::deduplicate_edges(self.iter_edges())
    }

    /// Iterate over the edges around the node at the specified position, in arbitrary order.
    ///
    /// Bidirectional edges like `SwapsWith` and `WorksWith` will be duplicated.
    /// To get deduplicated node edges, use `iter_edges_from_unique`.
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
    pub fn iter_edges_from_unique(
        &self,
        position: Position,
    ) -> impl Iterator<Item = EdgeView> + '_ {
        Self::deduplicate_edges(self.iter_edges_from(position))
    }

    fn deduplicate_edges(
        iterator: impl Iterator<Item = EdgeView>,
    ) -> impl Iterator<Item = EdgeView> {
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

    pub fn iter_semantic_edges_out_from(
        &self,
        position: Position,
    ) -> impl Iterator<Item = EdgeView> + '_ {
        self.iter_edges_from_unique(position)
            .filter(|edge| edge.r#type().is_semantic())
    }

    pub fn iter_semantic_neighbors_from(
        &self,
        position: Position,
    ) -> impl Iterator<Item = Position> + '_ {
        let mut seen = HashSet::new();

        let outgoing = self
            .edges_out
            .get(&position)
            .into_iter()
            .flatten()
            .filter(move |edge| edge.edge_type.is_semantic())
            .map(|edge| edge.other);

        let incoming = self
            .edges_in
            .get(&position)
            .into_iter()
            .flatten()
            .filter(move |edge| edge.edge_type.is_semantic())
            .map(|edge| edge.other);

        outgoing
            .chain(incoming)
            .filter(move |position| seen.insert(*position))
    }
}
