use crate::{
    domain::{EdgeType, GateType, Position, QuantumGraph},
    utils::math,
    view::{GraphNodeView, NodeEdgeView},
};

pub(crate) struct GraphAsserter<'a> {
    graph: &'a QuantumGraph,
}

impl<'a> GraphAsserter<'a> {
    pub(crate) fn new(graph: &'a QuantumGraph) -> Self {
        Self { graph }
    }

    pub(crate) fn is_empty(&self) -> &Self {
        assert!(self.graph.is_empty());
        &self
    }

    pub(crate) fn has_size(&self, size: usize) -> &Self {
        assert_eq!(self.graph.size(), size);
        &self
    }

    pub(crate) fn has_width(&self, width: usize) -> &Self {
        assert_eq!(self.graph.width(), width);
        &self
    }

    pub(crate) fn has_height(&self, height: usize) -> &Self {
        assert_eq!(self.graph.height(), height);
        &self
    }

    pub(crate) fn has_bits(&self, bits: usize) -> &Self {
        assert_eq!(self.graph.bits(), bits);
        &self
    }

    pub(crate) fn node_at(&self, position: Position) -> NodeAsserter<'a> {
        NodeAsserter {
            graph: self.graph,
            position,
        }
    }
}

pub(crate) struct NodeAsserter<'a> {
    graph: &'a QuantumGraph,
    position: Position,
}

impl<'a> NodeAsserter<'a> {
    pub(crate) fn is(self, gate: GateType) -> Self {
        assert_eq!(self.view().r#type(), gate);
        self
    }

    pub(crate) fn has_angle(self, angle: f64) -> Self {
        assert!(math::are_floats_similar(
            self.view().angle().unwrap(),
            angle
        ));
        self
    }

    pub(crate) fn has_no_angle(self) -> Self {
        assert!(self.view().angle().is_none());
        self
    }

    pub(crate) fn has_bit(self, bit: usize) -> Self {
        assert_eq!(self.view().bit(), Some(bit));
        self
    }

    pub(crate) fn has_no_bit(self) -> Self {
        assert!(self.view().bit().is_none());
        self
    }

    pub(crate) fn has_left(self, expected: Position) -> Self {
        let actual = self.edge_view().left().as_ref().map(|node| node.position());
        assert_eq!(actual, Some(expected));
        self
    }

    pub(crate) fn has_no_left(self) -> Self {
        assert!(self.edge_view().left().is_none());
        self
    }

    pub(crate) fn has_right(self, expected: Position) -> Self {
        let actual = self
            .edge_view()
            .right()
            .as_ref()
            .map(|node| node.position());
        assert_eq!(actual, Some(expected));
        self
    }

    pub(crate) fn has_no_right(self) -> Self {
        assert!(self.edge_view().right().is_none());
        self
    }

    pub(crate) fn targets(self, expected: &[Position]) -> Self {
        self.compare_positions(
            self.edge_view()
                .targets()
                .iter()
                .map(|node| node.position()),
            expected,
        );
        self
    }

    pub(crate) fn is_controlled_by(self, expected: &[Position]) -> Self {
        self.compare_positions(
            self.graph
                .iter_node_edges(self.position)
                .filter(|edge| edge.r#type() == EdgeType::ControlledBy)
                .map(|edge| edge.end().position()),
            expected,
        );
        self
    }

    pub(crate) fn works_with(self, expected: &[Position]) -> Self {
        self.compare_positions(
            self.edge_view()
                .works_with()
                .iter()
                .map(|node| node.position()),
            expected,
        );
        self
    }

    pub(crate) fn swaps_with(self, expected: Position) -> Self {
        let actual = self
            .edge_view()
            .swaps_with()
            .as_ref()
            .map(|node| node.position());
        assert_eq!(actual, Some(expected));
        self
    }

    pub(crate) fn swaps_with_none(self) -> Self {
        assert!(self.edge_view().swaps_with().is_none());
        self
    }

    fn compare_positions<I>(&self, actual: I, expected: &[Position])
    where
        I: Iterator<Item = Position>,
    {
        let mut actual: Vec<_> = actual.collect();
        let mut expected = expected.to_vec();

        sort(&mut actual);
        sort(&mut expected);

        assert_eq!(actual, expected);
    }

    fn view(&self) -> GraphNodeView {
        self.graph.get_node_view(self.position).unwrap()
    }

    fn edge_view(&self) -> NodeEdgeView {
        self.graph.node_edge_view(self.position).unwrap()
    }
}

fn sort(positions: &mut [Position]) {
    positions.sort_unstable_by_key(|position| (position.row(), position.column()));
}
