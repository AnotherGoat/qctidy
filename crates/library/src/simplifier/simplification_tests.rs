use crate::{GateOperation, GateType, GraphBuilder, simplifier::simplifier_mother};

#[test]
fn angles_very_close_to_zero_are_removed() {
    let mut graph = GraphBuilder::new(2)
        .push_p(0.0, 0)
        .unwrap()
        .push_rx(1e-8, 0)
        .unwrap()
        .push_ry(1e-9, 0)
        .unwrap()
        .push_rz(-1e-8, 0)
        .unwrap()
        .push_cp(-1e-9, 0, 1)
        .unwrap()
        .build();

    simplifier_mother::default().simplify(&mut graph, 10);

    let nodes: Vec<_> = graph.iter_nodes_ordered_by_column().collect();
    assert_eq!(nodes.len(), 0);
}

#[test]
fn angles_close_to_zero_are_kept() {
    let mut graph = GraphBuilder::new(2)
        .push_p(1.0, 0)
        .unwrap()
        .push_rx(1e-7, 0)
        .unwrap()
        .push_ry(1e-6, 0)
        .unwrap()
        .push_rz(-1e-7, 0)
        .unwrap()
        .push_cp(-1e-6, 0, 1)
        .unwrap()
        .build();

    simplifier_mother::default().simplify(&mut graph, 10);

    assert_eq!(graph.iter_nodes_ordered_by_column().count(), 6);
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

    simplifier_mother::default().simplify(&mut graph, 10);

    let mut nodes: Vec<_> = graph
        .iter_nodes_ordered_by_row()
        .map(|node| (node.position().row(), node.r#type()))
        .collect();
    nodes.sort_by_key(|(row, _)| *row);

    assert_eq!(nodes.len(), 2);
    assert_eq!(nodes[0], (0, GateType::X));
    assert_eq!(nodes[1], (1, GateType::X));
}

#[test]
fn double_x_with_gap_compacts_and_cancels() {
    let mut graph = GraphBuilder::new(1)
        .push_operation(&GateOperation::x(0))
        .push_operation(&GateOperation::id(0))
        .push_operation(&GateOperation::x(0))
        .build();

    simplifier_mother::default().simplify(&mut graph, 10);

    assert_eq!(graph.iter_nodes_ordered_by_column().count(), 0);
}
