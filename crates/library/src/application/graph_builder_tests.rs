use std::f64::consts::{FRAC_PI_2, FRAC_PI_3, PI};

use crate::{GateType, Position, domain::graph::graph_asserter::GraphAsserter};

use super::graph_builder::*;
use GateType::*;

#[test]
fn empty_graph_builder_builds_empty_valid_graph() {
    let graph = GraphBuilder::new().build();
    let asserter = GraphAsserter::new(&graph);

    asserter
        .is_empty()
        .has_size(0)
        .has_width(0)
        .has_height(0)
        .has_bits(0);

    graph.validate_internal();
}

#[test]
fn push_id_is_noop() {
    let graph = GraphBuilder::new().push_id(0).push_id(0).push_id(1).build();
    let asserter = GraphAsserter::new(&graph);

    asserter
        .is_empty()
        .has_size(0)
        .has_width(0)
        .has_height(0)
        .has_bits(0);

    graph.validate_internal();
}

#[test]
fn push_single_gate_calculates_column() {
    let graph = GraphBuilder::new().push_h(0).push_x(0).build();
    let asserter = GraphAsserter::new(&graph);

    asserter.has_size(2).has_width(2).has_height(1).has_bits(0);

    asserter.node_at(Position::new(0, 0)).is(H);
    asserter.node_at(Position::new(0, 1)).is(X);

    graph.validate_internal();
}

#[test]
fn push_single_gates_links_row_neighbors() {
    let graph = GraphBuilder::new().push_x(0).push_y(0).push_z(0).build();
    let asserter = GraphAsserter::new(&graph);

    asserter.has_size(3).has_width(3).has_height(1).has_bits(0);

    let x = Position::new(0, 0);
    let y = Position::new(0, 1);
    let z = Position::new(0, 2);

    asserter.node_at(x).has_right(y);

    asserter.node_at(y).has_right(z);

    asserter.node_at(z).has_no_right();

    graph.validate_internal();
}

#[test]
fn push_single_gates_in_different_rows() {
    let graph = GraphBuilder::new()
        .push_s(0)
        .push_sdg(0)
        .push_sx(1)
        .push_sy(1)
        .push_t(2)
        .push_tdg(2)
        .build();
    let asserter = GraphAsserter::new(&graph);

    asserter.has_size(6).has_width(2).has_height(3).has_bits(0);

    let s = Position::new(0, 0);
    let sdg = Position::new(0, 1);
    let sx = Position::new(1, 0);
    let sy = Position::new(1, 1);
    let t = Position::new(2, 0);
    let tdg = Position::new(2, 1);

    asserter.node_at(s).is(S).has_right(sdg);

    asserter.node_at(sdg).is(SDG).has_no_right();

    asserter.node_at(sx).is(SX).has_right(sy);

    asserter.node_at(sy).is(SY).has_no_right();

    asserter.node_at(t).is(T).has_right(tdg);

    asserter.node_at(tdg).is(TDG).has_no_right();

    graph.validate_internal();
}

#[test]
#[expect(clippy::unwrap_used)]
fn push_rotation_and_measure_gates_store_payloads() {
    let graph = GraphBuilder::new()
        .push_p(FRAC_PI_2, 0)
        .unwrap()
        .push_rx(PI, 0)
        .unwrap()
        .push_ry(-FRAC_PI_2, 1)
        .unwrap()
        .push_rz(FRAC_PI_3, 1)
        .unwrap()
        .push_measure(2, 0)
        .build();
    let asserter = GraphAsserter::new(&graph);

    asserter.has_size(5).has_width(2).has_height(3).has_bits(1);

    asserter
        .node_at(Position::new(0, 0))
        .is(P)
        .has_angle(FRAC_PI_2)
        .has_no_bit();

    asserter
        .node_at(Position::new(0, 1))
        .is(RX)
        .has_angle(PI)
        .has_no_bit();

    asserter
        .node_at(Position::new(1, 0))
        .is(RY)
        .has_angle(-FRAC_PI_2)
        .has_no_bit();

    asserter
        .node_at(Position::new(1, 1))
        .is(RZ)
        .has_angle(FRAC_PI_3)
        .has_no_bit();

    asserter
        .node_at(Position::new(2, 0))
        .is(Measure)
        .has_no_angle()
        .has_bit(0);

    graph.validate_internal();
}

#[test]
#[expect(clippy::unwrap_used)]
fn push_control_creates_multiple_nodes() {
    let graph = GraphBuilder::new().push_ch(0, 1).unwrap().build();
    let asserter = GraphAsserter::new(&graph);

    asserter.has_size(2).has_width(1).has_height(2).has_bits(0);

    asserter.node_at(Position::new(0, 0)).is(CH);
    asserter.node_at(Position::new(1, 0)).is(CH);

    graph.validate_internal();
}

#[test]
#[expect(clippy::unwrap_used)]
fn push_control_creates_relationships() {
    let graph = GraphBuilder::new()
        .push_cx(0, 1)
        .unwrap()
        .push_cy(1, 0)
        .unwrap()
        .build();
    let asserter = GraphAsserter::new(&graph);

    asserter.has_size(4).has_width(2).has_height(2).has_bits(0);

    let left_control = Position::new(0, 0);
    let left_target = Position::new(1, 0);

    asserter
        .node_at(left_control)
        .is(CX)
        .targets(&[left_target]);

    asserter.node_at(left_target).is(CX).targets_none();

    let right_control = Position::new(1, 1);
    let right_target = Position::new(0, 1);

    asserter
        .node_at(right_control)
        .is(CY)
        .targets(&[right_target]);

    asserter.node_at(right_target).is(CY).targets_none();

    graph.validate_internal();
}

#[test]
#[expect(clippy::unwrap_used)]
fn push_swap_builds_swap_pair() {
    let graph = GraphBuilder::new().push_swap(0, 1).unwrap().build();
    let asserter = GraphAsserter::new(&graph);

    asserter.has_size(2).has_width(1).has_height(2).has_bits(0);

    let node1 = Position::new(0, 0);
    let node2 = Position::new(1, 0);

    asserter.node_at(node1).is(Swap).swaps_with(node2);
    asserter.node_at(node2).is(Swap).swaps_with(node1);

    graph.validate_internal();
}

#[test]
#[expect(clippy::unwrap_used)]
fn push_cz_builds_symmetric_edges() {
    let graph = GraphBuilder::new()
        .push_cz(0, 1)
        .unwrap()
        .push_cz(1, 0)
        .unwrap()
        .build();
    let asserter = GraphAsserter::new(&graph);

    asserter.has_size(4).has_width(2).has_height(2).has_bits(0);

    let left1 = Position::new(0, 0);
    let left2 = Position::new(1, 0);

    asserter.node_at(left1).is(CZ).works_with(&[left2]);

    asserter.node_at(left2).is(CZ).works_with(&[left1]);

    let right1 = Position::new(0, 1);
    let right2 = Position::new(1, 1);

    asserter.node_at(right1).is(CZ).works_with(&[right2]);

    asserter.node_at(right2).is(CZ).works_with(&[right1]);

    graph.validate_internal();
}

#[test]
#[expect(clippy::unwrap_used)]
fn push_cp_builds_edges_and_payload() {
    let graph = GraphBuilder::new()
        .push_cp(PI, 0, 1)
        .unwrap()
        .push_cp(-PI, 1, 0)
        .unwrap()
        .build();
    let asserter = GraphAsserter::new(&graph);

    asserter.has_size(4).has_width(2).has_height(2).has_bits(0);

    let left1 = Position::new(0, 0);
    let left2 = Position::new(1, 0);

    asserter
        .node_at(left1)
        .is(CP)
        .has_angle(PI)
        .works_with(&[left2]);

    asserter
        .node_at(left2)
        .is(CP)
        .has_angle(PI)
        .works_with(&[left1]);

    let right1 = Position::new(0, 1);
    let right2 = Position::new(1, 1);

    asserter
        .node_at(right2)
        .is(CP)
        .has_angle(-PI)
        .works_with(&[right1]);

    asserter
        .node_at(right1)
        .is(CP)
        .has_angle(-PI)
        .works_with(&[right2]);

    graph.validate_internal();
}

#[test]
#[expect(clippy::unwrap_used)]
fn push_cswap_builds_control_and_swap_edges() {
    let graph = GraphBuilder::new().push_cswap(1, 0, 2).unwrap().build();
    let asserter = GraphAsserter::new(&graph);

    asserter.has_size(3).has_width(1).has_height(3).has_bits(0);

    let control = Position::new(1, 0);
    let target1 = Position::new(0, 0);
    let target2 = Position::new(2, 0);

    asserter
        .node_at(control)
        .is(CSwap)
        .targets(&[target1, target2]);

    asserter
        .node_at(target1)
        .is(CSwap)
        .targets_none()
        .swaps_with(target2);

    asserter
        .node_at(target2)
        .is(CSwap)
        .targets_none()
        .swaps_with(target1);

    graph.validate_internal();
}

#[test]
#[expect(clippy::unwrap_used)]
fn push_ccx_builds_two_control_qubits() {
    let graph = GraphBuilder::new().push_ccx(2, 0, 1).unwrap().build();
    let asserter = GraphAsserter::new(&graph);

    asserter.has_size(3).has_width(1).has_height(3).has_bits(0);

    let control1 = Position::new(2, 0);
    let control2 = Position::new(0, 0);
    let target = Position::new(1, 0);

    asserter
        .node_at(control1)
        .is(CCX)
        .targets(&[target])
        .works_with(&[control2]);

    asserter
        .node_at(control2)
        .is(CCX)
        .targets(&[target])
        .works_with(&[control1]);

    asserter.node_at(target).is(CCX).targets_none();

    graph.validate_internal();
}

#[test]
#[expect(clippy::unwrap_used)]
fn push_ccz_builds_triple_symmetric_edges() {
    let graph = GraphBuilder::new().push_ccz(2, 1, 0).unwrap().build();
    let asserter = GraphAsserter::new(&graph);

    asserter.has_size(3).has_width(1).has_height(3).has_bits(0);

    let node1 = Position::new(2, 0);
    let node2 = Position::new(1, 0);
    let node3 = Position::new(0, 0);

    asserter.node_at(node1).is(CCZ).works_with(&[node2, node3]);

    asserter.node_at(node2).is(CCZ).works_with(&[node1, node3]);

    asserter.node_at(node3).is(CCZ).works_with(&[node1, node2]);

    graph.validate_internal();
}

#[test]
#[expect(clippy::panic)]
fn push_p_rejects_infinity() {
    let mut builder1 = GraphBuilder::new();
    let positive = builder1.push_p(f64::INFINITY, 0);

    match positive {
        Err(GraphBuilderError::NonFiniteAngle { angle }) => {
            assert!(angle.is_infinite());
        }
        _ => panic!("Expected NonFiniteAngle error, got: {positive:?}"),
    }

    let mut builder2 = GraphBuilder::new();
    let negative = builder2.push_p(f64::NEG_INFINITY, 0);

    match negative {
        Err(GraphBuilderError::NonFiniteAngle { angle }) => {
            assert!(angle.is_infinite());
        }
        _ => panic!("Expected NonFiniteAngle error, got: {negative:?}"),
    }
}

#[test]
#[expect(clippy::panic)]
fn push_p_rejects_nan() {
    let mut builder = GraphBuilder::new();
    let result = builder.push_p(f64::NAN, 0);

    match result {
        Err(GraphBuilderError::NonFiniteAngle { angle }) => assert!(angle.is_nan()),
        _ => panic!("Expected NonFiniteAngle error, got: {result:?}"),
    }
}

#[test]
#[expect(clippy::panic)]
fn push_rx_rejects_infinity() {
    let mut builder1 = GraphBuilder::new();
    let positive = builder1.push_rx(f64::INFINITY, 0);

    match positive {
        Err(GraphBuilderError::NonFiniteAngle { angle }) => {
            assert!(angle.is_infinite());
        }
        _ => panic!("Expected NonFiniteAngle error, got: {positive:?}"),
    }

    let mut builder2 = GraphBuilder::new();
    let negative = builder2.push_rx(f64::NEG_INFINITY, 0);

    match negative {
        Err(GraphBuilderError::NonFiniteAngle { angle }) => {
            assert!(angle.is_infinite());
        }
        _ => panic!("Expected NonFiniteAngle error, got: {negative:?}"),
    }
}

#[test]
#[expect(clippy::panic)]
fn push_rx_rejects_nan() {
    let mut builder = GraphBuilder::new();
    let result = builder.push_rx(f64::NAN, 0);

    match result {
        Err(GraphBuilderError::NonFiniteAngle { angle }) => assert!(angle.is_nan()),
        _ => panic!("Expected NonFiniteAngle error, got: {result:?}"),
    }
}

#[test]
#[expect(clippy::panic)]
fn push_ry_rejects_infinity() {
    let mut builder1 = GraphBuilder::new();
    let positive = builder1.push_ry(f64::INFINITY, 0);

    match positive {
        Err(GraphBuilderError::NonFiniteAngle { angle }) => {
            assert!(angle.is_infinite());
        }
        _ => panic!("Expected NonFiniteAngle error, got: {positive:?}"),
    }

    let mut builder2 = GraphBuilder::new();
    let negative = builder2.push_ry(f64::NEG_INFINITY, 0);

    match negative {
        Err(GraphBuilderError::NonFiniteAngle { angle }) => {
            assert!(angle.is_infinite());
        }
        _ => panic!("Expected NonFiniteAngle error, got: {negative:?}"),
    }
}

#[test]
#[expect(clippy::panic)]
fn push_ry_rejects_nan() {
    let mut builder = GraphBuilder::new();
    let result = builder.push_ry(f64::NAN, 0);

    match result {
        Err(GraphBuilderError::NonFiniteAngle { angle }) => assert!(angle.is_nan()),
        _ => panic!("Expected NonFiniteAngle error, got: {result:?}"),
    }
}

#[test]
#[expect(clippy::panic)]
fn push_rz_rejects_infinity() {
    let mut builder1 = GraphBuilder::new();
    let positive = builder1.push_rz(f64::INFINITY, 0);

    match positive {
        Err(GraphBuilderError::NonFiniteAngle { angle }) => {
            assert!(angle.is_infinite());
        }
        _ => panic!("Expected NonFiniteAngle error, got: {positive:?}"),
    }

    let mut builder2 = GraphBuilder::new();
    let negative = builder2.push_rz(f64::NEG_INFINITY, 0);

    match negative {
        Err(GraphBuilderError::NonFiniteAngle { angle }) => {
            assert!(angle.is_infinite());
        }
        _ => panic!("Expected NonFiniteAngle error, got: {negative:?}"),
    }
}

#[test]
#[expect(clippy::panic)]
fn push_rz_rejects_nan() {
    let mut builder = GraphBuilder::new();
    let result = builder.push_rz(f64::NAN, 0);

    match result {
        Err(GraphBuilderError::NonFiniteAngle { angle }) => assert!(angle.is_nan()),
        _ => panic!("Expected NonFiniteAngle error, got: {result:?}"),
    }
}

#[test]
#[expect(clippy::panic)]
fn push_swap_rejects_same_qubit() {
    let mut builder = GraphBuilder::new();
    let result = builder.push_swap(0, 0);

    match result {
        Err(GraphBuilderError::RepeatedQubits { qubits }) => assert_eq!(qubits, vec![0, 0]),
        _ => panic!("Expected RepeatedQubits error, got: {result:?}"),
    }
}

#[test]
#[expect(clippy::panic)]
fn push_ch_rejects_same_qubit() {
    let mut builder = GraphBuilder::new();
    let result = builder.push_ch(0, 0);

    match result {
        Err(GraphBuilderError::RepeatedQubits { qubits }) => assert_eq!(qubits, vec![0, 0]),
        _ => panic!("Expected RepeatedQubits error, got: {result:?}"),
    }
}

#[test]
#[expect(clippy::panic)]
fn push_cx_rejects_same_qubit() {
    let mut builder = GraphBuilder::new();
    let result = builder.push_cx(0, 0);

    match result {
        Err(GraphBuilderError::RepeatedQubits { qubits }) => assert_eq!(qubits, vec![0, 0]),
        _ => panic!("Expected RepeatedQubits error, got: {result:?}"),
    }
}

#[test]
#[expect(clippy::panic)]
fn push_cy_rejects_same_qubit() {
    let mut builder = GraphBuilder::new();
    let result = builder.push_cy(0, 0);

    match result {
        Err(GraphBuilderError::RepeatedQubits { qubits }) => assert_eq!(qubits, vec![0, 0]),
        _ => panic!("Expected RepeatedQubits error, got: {result:?}"),
    }
}

#[test]
#[expect(clippy::panic)]
fn push_cz_rejects_same_qubit() {
    let mut builder = GraphBuilder::new();
    let result = builder.push_cz(0, 0);

    match result {
        Err(GraphBuilderError::RepeatedQubits { qubits }) => assert_eq!(qubits, vec![0, 0]),
        _ => panic!("Expected RepeatedQubits error, got: {result:?}"),
    }
}

#[test]
#[expect(clippy::panic)]
fn push_cp_rejects_infinity() {
    let mut builder1 = GraphBuilder::new();
    let positive = builder1.push_cp(f64::INFINITY, 0, 1);

    match positive {
        Err(GraphBuilderError::NonFiniteAngle { angle }) => {
            assert!(angle.is_infinite());
        }
        _ => panic!("Expected NonFiniteAngle error, got: {positive:?}"),
    }

    let mut builder2 = GraphBuilder::new();
    let negative = builder2.push_cp(f64::NEG_INFINITY, 0, 1);

    match negative {
        Err(GraphBuilderError::NonFiniteAngle { angle }) => {
            assert!(angle.is_infinite());
        }
        _ => panic!("Expected NonFiniteAngle error, got: {negative:?}"),
    }
}

#[test]
#[expect(clippy::panic)]
fn push_cp_rejects_nan() {
    let mut builder = GraphBuilder::new();
    let result = builder.push_cp(f64::NAN, 0, 1);

    match result {
        Err(GraphBuilderError::NonFiniteAngle { angle }) => assert!(angle.is_nan()),
        _ => panic!("Expected NonFiniteAngle error, got: {result:?}"),
    }
}

#[test]
#[expect(clippy::panic)]
fn push_cp_rejects_same_qubit() {
    let mut builder = GraphBuilder::new();
    let result = builder.push_cp(PI, 0, 0);

    match result {
        Err(GraphBuilderError::RepeatedQubits { qubits }) => assert_eq!(qubits, vec![0, 0]),
        _ => panic!("Expected RepeatedQubits error, got: {result:?}"),
    }
}

#[test]
#[expect(clippy::panic)]
fn push_cswap_rejects_repeated_qubits() {
    let mut builder1 = GraphBuilder::new();
    let first_two = builder1.push_cswap(0, 0, 1);

    match first_two {
        Err(GraphBuilderError::RepeatedQubits { qubits }) => assert_eq!(qubits, vec![0, 0, 1]),
        _ => panic!("Expected RepeatedQubits error, got: {first_two:?}"),
    }

    let mut builder2 = GraphBuilder::new();
    let last_two = builder2.push_cswap(1, 0, 0);

    match last_two {
        Err(GraphBuilderError::RepeatedQubits { qubits }) => assert_eq!(qubits, vec![1, 0, 0]),
        _ => panic!("Expected RepeatedQubits error, got: {last_two:?}"),
    }

    let mut builder3 = GraphBuilder::new();
    let extremes = builder3.push_cswap(0, 1, 0);

    match extremes {
        Err(GraphBuilderError::RepeatedQubits { qubits }) => assert_eq!(qubits, vec![0, 1, 0]),
        _ => panic!("Expected RepeatedQubits error, got: {extremes:?}"),
    }

    let mut builder4 = GraphBuilder::new();
    let all = builder4.push_cswap(0, 0, 0);

    match all {
        Err(GraphBuilderError::RepeatedQubits { qubits }) => assert_eq!(qubits, vec![0, 0, 0]),
        _ => panic!("Expected RepeatedQubits error, got: {all:?}"),
    }
}

#[test]
#[expect(clippy::panic)]
fn push_ccx_rejects_repeated_qubits() {
    let mut builder1 = GraphBuilder::new();
    let first_two = builder1.push_ccx(1, 1, 0);

    match first_two {
        Err(GraphBuilderError::RepeatedQubits { qubits }) => assert_eq!(qubits, vec![1, 1, 0]),
        _ => panic!("Expected RepeatedQubits error, got: {first_two:?}"),
    }

    let mut builder2 = GraphBuilder::new();
    let last_two = builder2.push_ccx(0, 1, 1);

    match last_two {
        Err(GraphBuilderError::RepeatedQubits { qubits }) => assert_eq!(qubits, vec![0, 1, 1]),
        _ => panic!("Expected RepeatedQubits error, got: {last_two:?}"),
    }

    let mut builder3 = GraphBuilder::new();
    let extremes = builder3.push_ccx(1, 0, 1);

    match extremes {
        Err(GraphBuilderError::RepeatedQubits { qubits }) => assert_eq!(qubits, vec![1, 0, 1]),
        _ => panic!("Expected RepeatedQubits error, got: {extremes:?}"),
    }

    let mut builder4 = GraphBuilder::new();
    let all = builder4.push_ccx(1, 1, 1);

    match all {
        Err(GraphBuilderError::RepeatedQubits { qubits }) => assert_eq!(qubits, vec![1, 1, 1]),
        _ => panic!("Expected RepeatedQubits error, got: {all:?}"),
    }
}

#[test]
#[expect(clippy::panic)]
fn push_ccz_rejects_repeated_qubits() {
    let mut builder1 = GraphBuilder::new();
    let first_two = builder1.push_ccz(1, 1, 2);

    match first_two {
        Err(GraphBuilderError::RepeatedQubits { qubits }) => assert_eq!(qubits, vec![1, 1, 2]),
        _ => panic!("Expected RepeatedQubits error, got: {first_two:?}"),
    }

    let mut builder2 = GraphBuilder::new();
    let last_two = builder2.push_ccz(2, 1, 1);

    match last_two {
        Err(GraphBuilderError::RepeatedQubits { qubits }) => assert_eq!(qubits, vec![2, 1, 1]),
        _ => panic!("Expected RepeatedQubits error, got: {last_two:?}"),
    }

    let mut builder3 = GraphBuilder::new();
    let extremes = builder3.push_ccz(1, 2, 1);

    match extremes {
        Err(GraphBuilderError::RepeatedQubits { qubits }) => assert_eq!(qubits, vec![1, 2, 1]),
        _ => panic!("Expected RepeatedQubits error, got: {extremes:?}"),
    }

    let mut builder4 = GraphBuilder::new();
    let all = builder4.push_ccz(1, 1, 1);

    match all {
        Err(GraphBuilderError::RepeatedQubits { qubits }) => assert_eq!(qubits, vec![1, 1, 1]),
        _ => panic!("Expected RepeatedQubits error, got: {all:?}"),
    }
}
