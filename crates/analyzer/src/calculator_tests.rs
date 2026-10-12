use qctidy::math::are_floats_equal;
use qctidy::{Graph, GraphBuilder};

use crate::calculator::{calculate_detailed_metrics, calculate_metrics};

#[test]
fn calculates_expected_metrics() {
    let mut builder = GraphBuilder::new(2);
    builder.push_h(0);
    builder.push_cx(0, 1).unwrap();
    let graph = builder.build();

    let metrics = calculate_metrics(&graph);

    assert_eq!(metrics.qubit_count, 2);
    assert_eq!(metrics.gate_count, 2);
    assert_eq!(metrics.hadamard_count, 1);
    assert_eq!(metrics.cx_count, 1);
    assert_eq!(metrics.x_count, 0);
    assert_eq!(metrics.single_gate_count, 1);
    assert_eq!(metrics.controlled_gate_count, 1);
}

#[test]
fn empty_graph_does_not_panic() {
    let graph = Graph::new(0);
    let detailed = calculate_detailed_metrics(&graph);
    assert!(are_floats_equal(detailed.average_density, 0.0));
    assert!(are_floats_equal(detailed.single_qubit_percent, 0.0));
    assert!(are_floats_equal(detailed.cnot_qubit_percent, 0.0));
}
