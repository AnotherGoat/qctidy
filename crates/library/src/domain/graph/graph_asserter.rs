use newgen::New;

use crate::{ContextualNodeView, GateType, Graph, NodeView, Position, domain::math};

#[derive(New)]
#[new(pub(crate), const)]
pub(crate) struct GraphAsserter<'a> {
    graph: &'a Graph,
}

impl<'a> GraphAsserter<'a> {
    /// Assert every basic property of the graph.
    ///
    /// This validates the graph (structure and semantics) and checks that the
    /// derived counts, dimensions, and node lookups are internally consistent.
    pub(crate) fn is_consistent(&self) -> &Self {
        self.graph
            .validate()
            .expect("The graph should be structurally and semantically valid");

        let node_count = self.graph.iter_nodes().count();

        assert_eq!(self.graph.node_count(), node_count);
        assert_eq!(
            self.graph.operation_count(),
            self.graph.gate_count() + self.graph.measurement_count()
        );
        assert_eq!(self.graph.is_empty(), node_count == 0);
        assert_eq!(self.graph.has_no_operations(), node_count == 0);

        for node in self.graph.iter_nodes() {
            let position = node.position();

            assert!(
                position.qubit() < self.graph.qubit_count(),
                "Node qubit must be inside the graph"
            );
            assert!(
                position.time_step() < self.graph.time_step_count(),
                "Node time step must be inside the graph"
            );
            assert!(self.graph.has_node_at(position));
            assert_eq!(self.graph.get_node(position), Some(node));
        }

        self
    }

    pub(crate) fn is_empty(&self) -> &Self {
        assert!(self.graph.is_empty());
        self
    }

    pub(crate) fn has_node_count(&self, node_count: usize) -> &Self {
        assert_eq!(self.graph.node_count(), node_count);
        self
    }

    pub(crate) fn has_operation_count(&self, operation_count: usize) -> &Self {
        assert_eq!(self.graph.operation_count(), operation_count);
        self
    }

    pub(crate) fn has_gate_count(&self, gate_count: usize) -> &Self {
        assert_eq!(self.graph.gate_count(), gate_count);
        self
    }

    pub(crate) fn has_measurement_count(&self, measurement_count: usize) -> &Self {
        assert_eq!(self.graph.measurement_count(), measurement_count);
        self
    }

    pub(crate) fn has_time_step_count(&self, time_step_count: usize) -> &Self {
        assert_eq!(self.graph.time_step_count(), time_step_count);
        self
    }

    pub(crate) fn has_qubit_count(&self, qubit_count: usize) -> &Self {
        assert_eq!(self.graph.qubit_count(), qubit_count);
        self
    }

    pub(crate) fn has_bit_count(&self, bit_count: usize) -> &Self {
        assert_eq!(self.graph.bit_count(), bit_count);
        self
    }

    pub(crate) fn node_at(&self, position: Position) -> NodeAsserter<'a> {
        assert!(
            self.graph.has_node_at(position),
            "Expected a node at {position:?}"
        );

        NodeAsserter {
            graph: self.graph,
            position,
        }
    }

    pub(crate) fn no_node_at(&self, position: Position) -> &Self {
        assert!(
            !self.graph.has_node_at(position),
            "Expected no node at {position:?}"
        );
        assert!(self.graph.get_node(position).is_none());

        self
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

    pub(crate) fn has_left(self, expected: Position) -> Self {
        let actual = self
            .contextual_view()
            .left()
            .as_ref()
            .map(NodeView::position);
        assert_eq!(actual, Some(expected));
        self
    }

    pub(crate) fn has_no_left(self) -> Self {
        assert!(self.contextual_view().left().is_none());
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

    pub(crate) fn swaps_with(self, expected: Position) -> Self {
        let actual = self
            .contextual_view()
            .swaps_with()
            .as_ref()
            .map(NodeView::position);
        assert_eq!(actual, Some(expected));
        self
    }

    pub(crate) fn controlled_by(self, expected: &[Position]) -> Self {
        compare_positions(
            self.contextual_view()
                .controlled_by()
                .iter()
                .map(NodeView::position),
            expected,
        );
        self
    }

    pub(crate) fn controlled_by_none(self) -> Self {
        assert!(self.contextual_view().controlled_by().is_empty());
        self
    }

    pub(crate) fn works_with_none(self) -> Self {
        assert!(self.contextual_view().works_with().is_empty());
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
    positions.sort_unstable_by_key(|position| (position.qubit(), position.time_step()));
}
