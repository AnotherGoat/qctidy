use newgen::New;

use crate::{ContextualNodeView, GateType, Graph, NodeView, Position, domain::math};

#[derive(New)]
#[new(pub(crate), const)]
pub(crate) struct GraphAsserter<'a> {
    graph: &'a Graph,
}

impl<'a> GraphAsserter<'a> {
    pub(crate) fn is_empty(&self) -> &Self {
        assert!(self.graph.is_empty());
        self
    }

    pub(crate) fn has_size(&self, size: usize) -> &Self {
        assert_eq!(self.graph.size(), size);
        self
    }

    pub(crate) fn has_width(&self, width: usize) -> &Self {
        assert_eq!(self.graph.width(), width);
        self
    }

    pub(crate) fn has_height(&self, height: usize) -> &Self {
        assert_eq!(self.graph.height(), height);
        self
    }

    pub(crate) fn has_bits(&self, bits: usize) -> &Self {
        assert_eq!(self.graph.bits(), bits);
        self
    }

    pub(crate) fn node_at(&self, position: Position) -> NodeAsserter<'a> {
        NodeAsserter {
            graph: self.graph,
            position,
        }
    }
}

pub(crate) struct NodeAsserter<'a> {
    graph: &'a Graph,
    position: Position,
}

impl NodeAsserter<'_> {
    pub(crate) fn is(self, gate: GateType) -> Self {
        assert_eq!(self.view().r#type(), gate);
        self
    }

    pub(crate) fn has_angle(self, angle: f64) -> Self {
        assert!(math::are_floats_equal(
            self.view().semantic_angle().unwrap(),
            angle
        ));
        self
    }

    pub(crate) fn has_no_angle(self) -> Self {
        assert!(self.view().semantic_angle().is_none());
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

    pub(crate) fn has_right(self, expected: Position) -> Self {
        let actual = self
            .contextual_view()
            .right()
            .as_ref()
            .map(NodeView::position);
        assert_eq!(actual, Some(expected));
        self
    }

    pub(crate) fn has_no_right(self) -> Self {
        assert!(self.contextual_view().right().is_none());
        self
    }

    pub(crate) fn targets(self, expected: &[Position]) -> Self {
        compare_positions(
            self.contextual_view()
                .targets()
                .iter()
                .map(NodeView::position),
            expected,
        );
        self
    }

    pub(crate) fn targets_none(self) -> Self {
        assert!(self.contextual_view().targets().is_empty());
        self
    }

    pub(crate) fn works_with(self, expected: &[Position]) -> Self {
        compare_positions(
            self.contextual_view()
                .works_with()
                .iter()
                .map(NodeView::position),
            expected,
        );
        self
    }

    pub(crate) fn works_with_none(self) -> Self {
        assert!(self.contextual_view().works_with().is_empty());
        self
    }

    pub(crate) fn swaps_with(self, expected: Position) -> Self {
        let actual = self
            .contextual_view()
            .swaps_with()
            .as_ref()
            .map(NodeView::position);
        assert_eq!(actual, Some(expected));
        self
    }

    pub(crate) fn swaps_with_none(self) -> Self {
        assert!(self.contextual_view().swaps_with().is_none());
        self
    }

    fn view(&self) -> NodeView {
        self.graph.get_node(self.position).unwrap()
    }

    fn contextual_view(&self) -> ContextualNodeView {
        self.graph.get_contextual_view(self.position).unwrap()
    }
}

fn compare_positions<I>(actual: I, expected: &[Position])
where
    I: Iterator<Item = Position>,
{
    let mut actual: Vec<_> = actual.collect();
    let mut expected = expected.to_vec();

    sort(&mut actual);
    sort(&mut expected);

    assert_eq!(actual, expected);
}

fn sort(positions: &mut [Position]) {
    positions.sort_unstable_by_key(|position| (position.row(), position.column()));
}
