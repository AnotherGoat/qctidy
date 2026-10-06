use std::collections::HashSet;

use crate::{Circuit, GateOperation, Graph, Position};

#[test]
fn operation_positions_match_the_built_graph() {
    let circuit = Circuit::new(
        3,
        vec![
            GateOperation::h(0),
            GateOperation::try_cx(0, 1).unwrap(),
            GateOperation::h(0),
            GateOperation::try_ccx(0, 1, 2).unwrap(),
            GateOperation::measure(2, 0),
        ],
    );

    let positions = circuit.operation_positions();
    let graph = Graph::from(&circuit);

    assert_eq!(positions.len(), circuit.operations().len());

    let from_operations: HashSet<Position> = positions.iter().flatten().copied().collect();
    let from_graph: HashSet<Position> = graph.iter_positions_ordered_by_row().collect();

    assert_eq!(from_operations, from_graph);
}

#[test]
fn operation_positions_advance_each_row_independently() {
    let circuit = Circuit::from_operations(vec![
        GateOperation::h(0),
        GateOperation::h(1),
        GateOperation::h(0),
    ]);

    let positions = circuit.operation_positions();

    assert_eq!(positions[0], vec![Position::new(0, 0)]);
    assert_eq!(positions[1], vec![Position::new(1, 0)]);
    assert_eq!(positions[2], vec![Position::new(0, 1)]);
}

#[test]
fn operation_positions_align_multi_qubit_gates() {
    let circuit = Circuit::from_operations(vec![
        GateOperation::try_cx(0, 1).unwrap(),
        GateOperation::h(0),
        GateOperation::h(1),
        GateOperation::h(2),
    ]);

    let positions = circuit.operation_positions();

    assert_eq!(positions[0], vec![Position::new(0, 0), Position::new(1, 0)]);
    assert_eq!(positions[1], vec![Position::new(0, 1)]);
    assert_eq!(positions[2], vec![Position::new(1, 1)]);
    assert_eq!(positions[3], vec![Position::new(2, 0)]);
}
