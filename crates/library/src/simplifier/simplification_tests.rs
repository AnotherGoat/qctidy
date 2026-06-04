use crate::{
    GateOperation, GraphBuilder, RuleConfiguration, RuleLevel,
    simplifier::{self, rule::registry::DEFAULT_RULE_REGISTRY},
};

fn build_simplifier() -> simplifier::Simplifier {
    simplifier::Simplifier::new(
        &DEFAULT_RULE_REGISTRY,
        vec![],
        &RuleConfiguration::new(RuleLevel::Apply),
    )
}

#[test]
fn x_on_control_between_cx_with_identities_compacts_and_propagates() {
    let mut graph = GraphBuilder::new(2)
        .push_operation(&GateOperation::try_cx(0, 1).unwrap())
        .push_operation(&GateOperation::x(0))
        .push_operation(&GateOperation::id(0))
        .push_operation(&GateOperation::id(1))
        .push_operation(&GateOperation::try_cx(0, 1).unwrap())
        .build();

    build_simplifier().simplify(&mut graph, 10);

    let mut nodes: Vec<_> = graph
        .iter_nodes_ordered_by_row()
        .map(|node| (node.position().row(), node.r#type()))
        .collect();
    nodes.sort_by_key(|(row, _)| *row);

    assert_eq!(nodes.len(), 2, "expected 2 X gates, got {nodes:?}");
    assert_eq!(nodes[0], (0, crate::GateType::X));
    assert_eq!(nodes[1], (1, crate::GateType::X));
}

#[test]
fn double_x_with_gap_compacts_and_cancels() {
    let mut graph = GraphBuilder::new(1)
        .push_operation(&GateOperation::x(0))
        .push_operation(&GateOperation::id(0))
        .push_operation(&GateOperation::x(0))
        .build();

    build_simplifier().simplify(&mut graph, 10);

    let nodes: Vec<_> = graph.iter_nodes_ordered_by_column().collect();
    assert_eq!(nodes.len(), 0, "expected empty graph, got {nodes:?}");
}
