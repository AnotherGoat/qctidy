use std::f64::consts::{FRAC_PI_2, FRAC_PI_3, PI};

use crate::domain::{GateType, Graph, GraphBuilder, Position};
use GateType::*;

#[test]
fn to_string_empty() {
    let graph = Graph::new();

    let result = graph.to_string();

    let expected = "\
Nodes:
(empty)

Edges:
(empty)";

    assert_eq!(result, expected);
}

#[test]
fn to_string_single_node() {
    let graph = GraphBuilder::new().push_h(0).build();

    let result = graph.to_string();

    let expected = "\
Nodes:
H at (0, 0)

Edges:
(empty)";

    assert_eq!(result, expected);
}

#[test]
fn to_string_skipping_first_row() {
    let graph = GraphBuilder::new().push_h(1).build();

    let result = graph.to_string();

    let expected = "\
Nodes:
H at (1, 0)

Edges:
(empty)";

    assert_eq!(result, expected);
}

#[test]
fn to_string_multiple_nodes_no_edges() {
    let graph = GraphBuilder::new().push_x(0).push_y(1).push_z(2).build();

    let result = graph.to_string();

    let expected = "\
Nodes:
X at (0, 0)
Y at (1, 0)
Z at (2, 0)

Edges:
(empty)";

    assert_eq!(result, expected);
}

#[test]
fn to_string_sparse_columns() {
    let mut graph = Graph::new();

    graph.add_node(H, Position::new(0, 0), None, None).unwrap();
    graph.add_node(X, Position::new(0, 2), None, None).unwrap();

    let result = graph.to_string();

    let expected = "\
Nodes:
H at (0, 0)
X at (0, 2)

Edges:
(empty)";

    assert_eq!(result, expected);
}

#[test]
fn to_string_rotation_gates() {
    let graph = GraphBuilder::new()
        .push_rx(FRAC_PI_2, 0)
        .push_ry(3.0 * PI, 1)
        .push_rz(2.0 * FRAC_PI_3, 2)
        .build();

    let result = graph.to_string();

    let expected = "\
Nodes:
RX(angle=pi/2) at (0, 0)
RY(angle=3pi) at (1, 0)
RZ(angle=2pi/3) at (2, 0)

Edges:
(empty)";

    assert_eq!(result, expected);
}

#[test]
fn to_string_measurement_gates() {
    let graph = GraphBuilder::new()
        .push_measure(0, 0)
        .push_measure(1, 2)
        .push_measure(2, 1)
        .build();

    let result = graph.to_string();

    let expected = "\
Nodes:
M(bit=0) at (0, 0)
M(bit=2) at (1, 0)
M(bit=1) at (2, 0)

Edges:
(empty)";

    assert_eq!(result, expected);
}

#[test]
fn to_string_linear_chain() {
    let graph = GraphBuilder::new().push_x(0).push_y(0).push_z(0).build();

    let result = graph.to_string();

    let expected = "\
Nodes:
X at (0, 0)
Y at (0, 1)
Z at (0, 2)

Edges:
[right] from X at (0, 0) to Y at (0, 1)

[right] from Y at (0, 1) to Z at (0, 2)";

    assert_eq!(result, expected);
}

#[test]
fn to_string_rows_and_columns() {
    let graph = GraphBuilder::new()
        .push_h(0)
        .push_x(0)
        .push_y(1)
        .push_z(1)
        .build();

    let result = graph.to_string();

    let expected = "\
Nodes:
H at (0, 0)
Y at (1, 0)
X at (0, 1)
Z at (1, 1)

Edges:
[right] from H at (0, 0) to X at (0, 1)

[right] from Y at (1, 0) to Z at (1, 1)";

    assert_eq!(result, expected);
}

#[test]
fn to_string_swap_cycle() {
    let graph = GraphBuilder::new().push_swap(0, 1).build();

    let result = graph.to_string();

    let expected = "\
Nodes:
SWAP at (0, 0)
SWAP at (1, 0)

Edges:
[swaps_with] from SWAP at (0, 0) to SWAP at (1, 0)

[swaps_with] from SWAP at (1, 0) to SWAP at (0, 0)";

    assert_eq!(result, expected);
}

#[test]
fn to_string_controlled_gates() {
    let graph = GraphBuilder::new().push_cx(0, 1).push_cy(1, 0).build();

    let result = graph.to_string();

    let expected = "\
Nodes:
CX at (0, 0)
CX at (1, 0)
CY at (0, 1)
CY at (1, 1)

Edges:
[right] from CX at (0, 0) to CY at (0, 1)
[targets] from CX at (0, 0) to CX at (1, 0)

[right] from CX at (1, 0) to CY at (1, 1)

[targets] from CY at (1, 1) to CY at (0, 1)";

    assert_eq!(result, expected);
}

#[test]
fn to_string_sorts_edges_of_same_type() {
    let graph = GraphBuilder::new()
        .push_ccx(1, 2, 0)
        .push_ccz(2, 0, 1)
        .build();

    let result = graph.to_string();

    let expected = "\
Nodes:
CCX at (0, 0)
CCX at (1, 0)
CCX at (2, 0)
CCZ at (0, 1)
CCZ at (1, 1)
CCZ at (2, 1)

Edges:
[right] from CCX at (0, 0) to CCZ at (0, 1)

[right] from CCX at (1, 0) to CCZ at (1, 1)
[targets] from CCX at (1, 0) to CCX at (0, 0)
[works_with] from CCX at (1, 0) to CCX at (2, 0)

[right] from CCX at (2, 0) to CCZ at (2, 1)
[targets] from CCX at (2, 0) to CCX at (0, 0)
[works_with] from CCX at (2, 0) to CCX at (1, 0)

[works_with] from CCZ at (0, 1) to CCZ at (1, 1)
[works_with] from CCZ at (0, 1) to CCZ at (2, 1)

[works_with] from CCZ at (1, 1) to CCZ at (0, 1)
[works_with] from CCZ at (1, 1) to CCZ at (2, 1)

[works_with] from CCZ at (2, 1) to CCZ at (0, 1)
[works_with] from CCZ at (2, 1) to CCZ at (1, 1)";

    assert_eq!(result, expected);
}

#[test]
fn display_grid_empty_graph() {
    let graph = Graph::new();

    let result = graph.display_grid();

    assert_eq!(result, "(empty)");
}

#[test]
fn display_grid_single_gate() {
    let graph = GraphBuilder::new().push_h(0).build();

    let result = graph.display_grid();

    let expected = "0: H";
    assert_eq!(result, expected);
}

#[test]
fn display_grid_skipping_first_row() {
    let graph = GraphBuilder::new().push_h(1).build();

    let result = graph.display_grid();

    let expected = "\
0: .
1: H";
    assert_eq!(result, expected);
}

#[test]
fn display_grid_multiple_rows_single_column() {
    let graph = GraphBuilder::new().push_h(0).push_x(1).push_z(2).build();

    let result = graph.display_grid();

    let expected = "\
0: H
1: X
2: Z";

    assert_eq!(result, expected);
}

#[test]
fn display_grid_sparse_columns() {
    let mut graph = Graph::new();

    graph.add_node(H, Position::new(0, 0), None, None).unwrap();
    graph.add_node(X, Position::new(0, 2), None, None).unwrap();

    let result = graph.display_grid();

    let expected = "0: H   .   X";
    assert_eq!(result, expected);
}

#[test]
fn display_grid_multiple_rows_and_columns() {
    let mut graph = Graph::new();

    graph.add_node(H, Position::new(0, 0), None, None).unwrap();
    graph.add_node(X, Position::new(1, 1), None, None).unwrap();
    graph.add_node(Z, Position::new(0, 2), None, None).unwrap();

    let result = graph.display_grid();

    let expected = "\
0: H   .   Z
1: .   X   .";

    assert_eq!(result, expected);
}

#[test]
fn display_grid_with_angle() {
    let graph = GraphBuilder::new()
        .push_rx(FRAC_PI_2, 0)
        .push_ry(3.0 * PI, 1)
        .push_rz(2.0 * FRAC_PI_3, 2)
        .build();

    let result = graph.display_grid();

    let expected = "\
0: RX(pi/2)
1: RY(3pi)
2: RZ(2pi/3)";

    assert_eq!(result, expected);
}

#[test]
fn display_grid_with_multiple_angles_alignment() {
    let graph = GraphBuilder::new()
        .push_rx(FRAC_PI_2, 0)
        .push_rz(0.0, 0)
        .build();

    let result = graph.display_grid();

    let expected = "0: RX(pi/2)   RZ(0)";
    assert_eq!(result, expected);
}

#[test]
fn display_grid_mixed_angle_and_normal() {
    let graph = GraphBuilder::new()
        .push_h(0)
        .push_rx(std::f64::consts::FRAC_PI_2, 0)
        .build();

    let result = graph.display_grid();

    let expected = "0: H   RX(pi/2)";
    assert_eq!(result, expected);
}

#[test]
fn display_grid_measure_with_bit() {
    let graph = GraphBuilder::new().push_measure(0, 3).build();

    let result = graph.display_grid();

    let expected = "0: M(3)";
    assert_eq!(result, expected);
}

#[test]
fn display_grid_column_alignment() {
    let graph = GraphBuilder::new()
        .push_h(0)
        .push_x(0)
        .push_measure(1, 0)
        .push_z(1)
        .build();

    let result = graph.display_grid();

    let expected = "\
0: H      X
1: M(0)   Z";

    assert_eq!(result, expected);
}

#[test]
fn display_grid_sparse_with_angles() {
    let mut graph = Graph::new();

    graph
        .add_node(
            RX,
            Position::new(0, 0),
            Some(std::f64::consts::FRAC_PI_2),
            None,
        )
        .unwrap();

    graph
        .add_node(Measure, Position::new(0, 2), None, Some(2))
        .unwrap();

    let result = graph.display_grid();

    let expected = "0: RX(pi/2)   .   M(2)";
    assert_eq!(result, expected);
}
