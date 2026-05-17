use std::f64::consts::{FRAC_PI_2, FRAC_PI_4, PI};

use faer::{Mat, complex::Complex64};

use crate::{
    EdgeType, GateType, Graph, GraphBuilder, Position,
    domain::math,
    simplifier::matrix_calculator::{are_graphs_equivalent, graph_circuit_matrix},
};
use EdgeType::*;
use GateType::*;

#[test]
fn zero_qubits_matrix() {
    let graph = Graph::default();

    let actual = graph_circuit_matrix(&graph);
    let expected = Mat::identity(1, 1);

    assert!(math::are_matrices_equal(&actual, &expected));
}

#[test]
fn empty_matrix() {
    let identity2 = Graph::new(1);

    assert!(math::are_matrices_equal(
        &graph_circuit_matrix(&identity2),
        &Mat::identity(2, 2)
    ));

    let identity4 = Graph::new(2);

    assert!(math::are_matrices_equal(
        &graph_circuit_matrix(&identity4),
        &Mat::identity(4, 4)
    ));

    let identity8 = Graph::new(3);

    assert!(math::are_matrices_equal(
        &graph_circuit_matrix(&identity8),
        &Mat::identity(8, 8)
    ));
}

#[test]
fn identity_matrix() {
    let mut identity2 = Graph::default();
    identity2.replace_node(ID, Position::new(0, 0), None, None);

    assert!(math::are_matrices_equal(
        &graph_circuit_matrix(&identity2),
        &Mat::identity(2, 2)
    ));

    let mut identity4 = Graph::default();
    identity4.replace_node(ID, Position::new(0, 0), None, None);
    identity4.replace_node(ID, Position::new(1, 0), None, None);

    assert!(math::are_matrices_equal(
        &graph_circuit_matrix(&identity4),
        &Mat::identity(4, 4)
    ));

    let mut identity8 = Graph::default();
    identity8.replace_node(ID, Position::new(0, 0), None, None);
    identity8.replace_node(ID, Position::new(1, 0), None, None);
    identity8.replace_node(ID, Position::new(2, 0), None, None);

    assert!(math::are_matrices_equal(
        &graph_circuit_matrix(&identity8),
        &Mat::identity(8, 8)
    ));
}

#[test]
#[expect(clippy::unnested_or_patterns)]
fn hadamard_matrix() {
    let graph = GraphBuilder::default().push_h(0).build();
    let half_sqrt = 1.0_f64 / 2.0_f64.sqrt();

    let actual = graph_circuit_matrix(&graph);

    let expected = Mat::from_fn(2, 2, |row, column| match (row, column) {
        (0, 0) | (0, 1) | (1, 0) => Complex64::from(half_sqrt),
        (1, 1) => Complex64::from(-half_sqrt),
        _ => Complex64::default(),
    });

    assert!(math::are_matrices_equal(&actual, &expected));
}

#[test]
fn x_matrix() {
    let graph = GraphBuilder::default().push_x(0).build();

    let actual = graph_circuit_matrix(&graph);
    let expected = Mat::from_fn(2, 2, |row, column| match (row, column) {
        (0, 1) | (1, 0) => Complex64::from(1.0_f64),
        _ => Complex64::default(),
    });

    assert!(math::are_matrices_equal(&actual, &expected));
}

#[test]
#[expect(clippy::unwrap_used)]
fn x_matrix_surrounded_by_identities() {
    let mut graph = Graph::default();

    graph.replace_node(ID, Position::new(0, 0), None, None);
    graph.replace_node(X, Position::new(0, 1), None, None);
    graph.replace_node(ID, Position::new(0, 2), None, None);

    graph
        .add_edge(Right, Position::new(0, 0), Position::new(0, 1))
        .unwrap();
    graph
        .add_edge(Right, Position::new(0, 1), Position::new(0, 2))
        .unwrap();

    let actual = graph_circuit_matrix(&graph);
    let expected = Mat::from_fn(2, 2, |row, column| match (row, column) {
        (0, 1) | (1, 0) => Complex64::from(1.0_f64),
        _ => Complex64::default(),
    });

    assert!(math::are_matrices_equal(&actual, &expected));
}

#[test]
fn y_matrix() {
    let graph = GraphBuilder::default().push_y(0).build();

    let actual = graph_circuit_matrix(&graph);
    let expected = Mat::from_fn(2, 2, |row, column| match (row, column) {
        (0, 1) => Complex64::new(0.0_f64, -1.0_f64),
        (1, 0) => Complex64::new(0.0_f64, 1.0_f64),
        _ => Complex64::default(),
    });

    assert!(math::are_matrices_equal(&actual, &expected));
}

#[test]
fn z_matrix() {
    let graph = GraphBuilder::default().push_z(0).build();

    let actual = graph_circuit_matrix(&graph);
    let expected = Mat::from_fn(2, 2, |row, column| match (row, column) {
        (0, 0) => Complex64::from(1.0_f64),
        (1, 1) => Complex64::from(-1.0_f64),
        _ => Complex64::default(),
    });

    assert!(math::are_matrices_equal(&actual, &expected));
}

#[test]
#[expect(clippy::unwrap_used)]
fn phase_matrix() {
    let graph = GraphBuilder::default()
        .push_p(FRAC_PI_2, 0)
        .unwrap()
        .build();

    let actual = graph_circuit_matrix(&graph);
    let expected = Mat::from_fn(2, 2, |row, column| match (row, column) {
        (0, 0) => Complex64::from(1.0_f64),
        (1, 1) => Complex64::new(0.0_f64, 1.0_f64),
        _ => Complex64::default(),
    });

    assert!(math::are_matrices_equal(&actual, &expected));
}

#[test]
#[expect(clippy::unwrap_used)]
fn rx_matrix() {
    let graph = GraphBuilder::default()
        .push_rx(FRAC_PI_2, 0)
        .unwrap()
        .build();
    let half_sqrt = 1.0_f64 / 2.0_f64.sqrt();

    let actual = graph_circuit_matrix(&graph);
    let expected = Mat::from_fn(2, 2, |row, column| match (row, column) {
        (0, 0) | (1, 1) => Complex64::from(half_sqrt),
        (0, 1) | (1, 0) => Complex64::new(0.0_f64, -half_sqrt),
        _ => Complex64::default(),
    });

    assert!(math::are_matrices_equal(&actual, &expected));
}

#[test]
#[expect(clippy::unnested_or_patterns, clippy::unwrap_used)]
fn ry_matrix() {
    let graph = GraphBuilder::default()
        .push_ry(FRAC_PI_2, 0)
        .unwrap()
        .build();
    let half_sqrt = 1.0_f64 / 2.0_f64.sqrt();

    let actual = graph_circuit_matrix(&graph);
    let expected = Mat::from_fn(2, 2, |row, column| match (row, column) {
        (0, 0) | (1, 0) | (1, 1) => Complex64::from(half_sqrt),
        (0, 1) => Complex64::from(-half_sqrt),
        _ => Complex64::default(),
    });

    assert!(math::are_matrices_equal(&actual, &expected));
}

#[test]
#[expect(clippy::unwrap_used)]
fn rz_matrix() {
    let graph = GraphBuilder::default()
        .push_rz(FRAC_PI_2, 0)
        .unwrap()
        .build();

    let actual = graph_circuit_matrix(&graph);
    let expected = Mat::from_fn(2, 2, |row, column| match (row, column) {
        (0, 0) => Complex64::new(0.0_f64, -FRAC_PI_4).exp(),
        (1, 1) => Complex64::new(0.0_f64, FRAC_PI_4).exp(),
        _ => Complex64::default(),
    });

    assert!(math::are_matrices_equal(&actual, &expected));
}

#[test]
fn s_matrix() {
    let graph = GraphBuilder::default().push_s(0).build();

    let actual = graph_circuit_matrix(&graph);
    let expected = Mat::from_fn(2, 2, |row, column| match (row, column) {
        (0, 0) => Complex64::from(1.0_f64),
        (1, 1) => Complex64::new(0.0_f64, 1.0_f64),
        _ => Complex64::default(),
    });

    assert!(math::are_matrices_equal(&actual, &expected));
}

#[test]
fn s_dagger_matrix() {
    let graph = GraphBuilder::default().push_sdg(0).build();

    let actual = graph_circuit_matrix(&graph);
    let expected = Mat::from_fn(2, 2, |row, column| match (row, column) {
        (0, 0) => Complex64::from(1.0_f64),
        (1, 1) => Complex64::new(0.0_f64, -1.0_f64),
        _ => Complex64::default(),
    });

    assert!(math::are_matrices_equal(&actual, &expected));
}

#[test]
fn sqrt_x_matrix() {
    let graph = GraphBuilder::default().push_sx(0).build();

    let actual = graph_circuit_matrix(&graph);
    let expected = 0.5_f64
        * Mat::from_fn(2, 2, |row, column| match (row, column) {
            (0, 0) | (1, 1) => Complex64::new(1.0_f64, 1.0_f64),
            (0, 1) | (1, 0) => Complex64::new(1.0_f64, -1.0_f64),
            _ => Complex64::default(),
        });

    assert!(math::are_matrices_equal(&actual, &expected));
}

#[test]
#[expect(clippy::unnested_or_patterns)]
fn sqrt_y_matrix() {
    let graph = GraphBuilder::default().push_sy(0).build();

    let actual = graph_circuit_matrix(&graph);
    let expected = 0.5_f64
        * Mat::from_fn(2, 2, |row, column| match (row, column) {
            (0, 0) | (1, 0) | (1, 1) => Complex64::new(1.0_f64, 1.0_f64),
            (0, 1) => Complex64::new(-1.0_f64, -1.0_f64),
            _ => Complex64::default(),
        });

    assert!(math::are_matrices_equal(&actual, &expected));
}

#[test]
fn t_matrix() {
    let graph = GraphBuilder::default().push_t(0).build();

    let actual = graph_circuit_matrix(&graph);
    let expected = Mat::from_fn(2, 2, |row, column| match (row, column) {
        (0, 0) => Complex64::from(1.0_f64),
        (1, 1) => (Complex64::new(0.0_f64, 1.0_f64) * PI / 4.0_f64).exp(),
        _ => Complex64::default(),
    });

    assert!(math::are_matrices_equal(&actual, &expected));
}

#[test]
fn t_dagger_matrix() {
    let graph = GraphBuilder::default().push_tdg(0).build();

    let actual = graph_circuit_matrix(&graph);
    let expected = Mat::from_fn(2, 2, |row, column| match (row, column) {
        (0, 0) => Complex64::from(1.0_f64),
        (1, 1) => (Complex64::new(0.0_f64, -1.0_f64) * PI / 4.0_f64).exp(),
        _ => Complex64::default(),
    });

    assert!(math::are_matrices_equal(&actual, &expected));
}

#[test]
fn empty_matrix_with_ignored_measurements() {
    let graph = GraphBuilder::default()
        .push_measure(0, 0)
        .push_measure(1, 1)
        .build();

    let actual = graph_circuit_matrix(&graph);
    let expected = Mat::identity(4, 4);

    assert!(math::are_matrices_equal(&actual, &expected));
}

#[test]
fn x_matrix_with_ignored_measurement() {
    let graph = GraphBuilder::default().push_x(0).push_measure(0, 0).build();

    let actual = graph_circuit_matrix(&graph);
    let expected = Mat::from_fn(2, 2, |row, column| match (row, column) {
        (0, 1) | (1, 0) => Complex64::from(1.0_f64),
        _ => Complex64::default(),
    });

    assert!(math::are_matrices_equal(&actual, &expected));
}

#[test]
#[expect(clippy::unwrap_used)]
fn swap_matrix() {
    let graph = GraphBuilder::default().push_swap(0, 1).unwrap().build();

    let actual = graph_circuit_matrix(&graph);
    let expected = Mat::from_fn(4, 4, |row, column| match (row, column) {
        (0, 0) | (1, 2) | (2, 1) | (3, 3) => Complex64::from(1.0_f64),
        _ => Complex64::default(),
    });

    assert!(math::are_matrices_equal(&actual, &expected));
}

#[test]
#[expect(clippy::unnested_or_patterns, clippy::unwrap_used)]
fn control_hadamard_matrix() {
    let graph = GraphBuilder::default().push_ch(0, 1).unwrap().build();
    let half_sqrt = 1.0_f64 / 2.0_f64.sqrt();

    let actual = graph_circuit_matrix(&graph);
    let expected = Mat::from_fn(4, 4, |row, column| match (row, column) {
        (0, 0) | (1, 1) => Complex64::from(1.0_f64),
        (2, 2) | (2, 3) | (3, 2) => Complex64::from(half_sqrt),
        (3, 3) => Complex64::from(-half_sqrt),
        _ => Complex64::default(),
    });

    assert!(math::are_matrices_equal(&actual, &expected));
}

#[test]
#[expect(clippy::unnested_or_patterns, clippy::unwrap_used)]
fn reversed_ch_matrix() {
    let graph = GraphBuilder::default().push_ch(1, 0).unwrap().build();
    let half_sqrt = 1.0_f64 / 2.0_f64.sqrt();

    let actual = graph_circuit_matrix(&graph);
    let expected = Mat::from_fn(4, 4, |row, column| match (row, column) {
        (0, 0) | (2, 2) => Complex64::from(1.0_f64),
        (1, 1) | (1, 3) | (3, 1) => Complex64::from(half_sqrt),
        (3, 3) => Complex64::from(-half_sqrt),
        _ => Complex64::default(),
    });

    assert!(math::are_matrices_equal(&actual, &expected));
}

#[test]
#[expect(clippy::unwrap_used)]
fn cx_matrix() {
    let graph = GraphBuilder::default().push_cx(0, 1).unwrap().build();

    let actual = graph_circuit_matrix(&graph);
    let expected = Mat::from_fn(4, 4, |row, column| match (row, column) {
        (0, 0) | (1, 1) | (2, 3) | (3, 2) => Complex64::from(1.0_f64),
        _ => Complex64::default(),
    });

    assert!(math::are_matrices_equal(&actual, &expected));
}

#[test]
#[expect(clippy::unwrap_used)]
fn reverse_cx_matrix() {
    let graph = GraphBuilder::default().push_cx(1, 0).unwrap().build();

    let actual = graph_circuit_matrix(&graph);
    let expected = Mat::from_fn(4, 4, |row, column| match (row, column) {
        (0, 0) | (1, 3) | (2, 2) | (3, 1) => Complex64::from(1.0_f64),
        _ => Complex64::default(),
    });

    assert!(math::are_matrices_equal(&actual, &expected));
}

#[test]
#[expect(clippy::unwrap_used)]
fn cy_matrix() {
    let graph = GraphBuilder::default().push_cy(0, 1).unwrap().build();

    let actual = graph_circuit_matrix(&graph);
    let expected = Mat::from_fn(4, 4, |row, column| match (row, column) {
        (0, 0) | (1, 1) => Complex64::from(1.0_f64),
        (2, 3) => Complex64::new(0.0_f64, -1.0_f64),
        (3, 2) => Complex64::new(0.0_f64, 1.0_f64),
        _ => Complex64::default(),
    });

    assert!(math::are_matrices_equal(&actual, &expected));
}

#[test]
#[expect(clippy::unwrap_used)]
fn reverse_cy_matrix() {
    let graph = GraphBuilder::default().push_cy(1, 0).unwrap().build();

    let actual = graph_circuit_matrix(&graph);
    let expected = Mat::from_fn(4, 4, |row, column| match (row, column) {
        (0, 0) | (2, 2) => Complex64::from(1.0_f64),
        (1, 3) => Complex64::new(0.0_f64, -1.0_f64),
        (3, 1) => Complex64::new(0.0_f64, 1.0_f64),
        _ => Complex64::default(),
    });

    assert!(math::are_matrices_equal(&actual, &expected));
}

#[test]
#[expect(clippy::unwrap_used)]
fn control_phase_matrix() {
    let graph = GraphBuilder::default()
        .push_cp(FRAC_PI_2, 0, 1)
        .unwrap()
        .build();

    let actual = graph_circuit_matrix(&graph);
    let expected = Mat::from_fn(4, 4, |row, column| match (row, column) {
        (0, 0) | (1, 1) | (2, 2) => Complex64::from(1.0_f64),
        (3, 3) => Complex64::new(0.0_f64, 1.0_f64),
        _ => Complex64::default(),
    });

    assert!(math::are_matrices_equal(&actual, &expected));
}

#[test]
#[expect(clippy::unwrap_used)]
fn cz_matrix() {
    let graph = GraphBuilder::default().push_cz(0, 1).unwrap().build();

    let actual = graph_circuit_matrix(&graph);
    let expected = Mat::from_fn(4, 4, |row, column| match (row, column) {
        (0, 0) | (1, 1) | (2, 2) => Complex64::from(1.0_f64),
        (3, 3) => Complex64::from(-1.0_f64),
        _ => Complex64::default(),
    });

    assert!(math::are_matrices_equal(&actual, &expected));
}

#[test]
#[expect(clippy::unwrap_used)]
fn cswap_matrix() {
    let graph = GraphBuilder::default().push_cswap(0, 1, 2).unwrap().build();

    let actual = graph_circuit_matrix(&graph);
    let expected = Mat::from_fn(8, 8, |row, column| match (row, column) {
        (0, 0) | (1, 1) | (2, 2) | (3, 3) | (4, 4) | (5, 6) | (6, 5) | (7, 7) => {
            Complex64::from(1.0_f64)
        }
        _ => Complex64::default(),
    });

    assert!(math::are_matrices_equal(&actual, &expected));
}

#[test]
#[expect(clippy::unwrap_used)]
fn ccx_matrix() {
    let graph = GraphBuilder::default().push_ccx(0, 1, 2).unwrap().build();

    let actual = graph_circuit_matrix(&graph);
    let expected = Mat::from_fn(8, 8, |row, column| match (row, column) {
        (0, 0) | (1, 1) | (2, 2) | (3, 3) | (4, 4) | (5, 5) | (6, 7) | (7, 6) => {
            Complex64::from(1.0_f64)
        }
        _ => Complex64::default(),
    });

    assert!(math::are_matrices_equal(&actual, &expected));
}

#[test]
#[expect(clippy::unwrap_used)]
fn ccz_matrix() {
    let graph = GraphBuilder::default().push_ccz(0, 1, 2).unwrap().build();

    let actual = graph_circuit_matrix(&graph);
    let expected = Mat::from_fn(8, 8, |row, column| match (row, column) {
        (0, 0) | (1, 1) | (2, 2) | (3, 3) | (4, 4) | (5, 5) | (6, 6) => Complex64::from(1.0_f64),
        (7, 7) => Complex64::from(-1.0_f64),
        _ => Complex64::default(),
    });

    assert!(math::are_matrices_equal(&actual, &expected));
}

#[test]
#[expect(clippy::unwrap_used)]
fn equivalent_swap_permutations() {
    let graph = GraphBuilder::default().push_swap(0, 1).unwrap().build();
    let graph2 = GraphBuilder::default().push_swap(1, 0).unwrap().build();

    assert!(are_graphs_equivalent(&graph, &graph2));
}

#[test]
#[expect(clippy::unwrap_used)]
fn equivalent_cz_permutations() {
    let graph = GraphBuilder::default().push_cz(0, 1).unwrap().build();
    let graph2 = GraphBuilder::default().push_cz(1, 0).unwrap().build();

    assert!(are_graphs_equivalent(&graph, &graph2));
}

#[test]
#[expect(clippy::unwrap_used)]
fn equivalent_cp_permutations() {
    let graph = GraphBuilder::default()
        .push_cp(FRAC_PI_2, 0, 1)
        .unwrap()
        .build();
    let graph2 = GraphBuilder::default()
        .push_cp(FRAC_PI_2, 1, 0)
        .unwrap()
        .build();

    assert!(are_graphs_equivalent(&graph, &graph2));
}

#[test]
#[expect(clippy::unwrap_used)]
fn equivalent_cswap_permutations() {
    let graph = GraphBuilder::default().push_cswap(0, 1, 2).unwrap().build();
    let graph2 = GraphBuilder::default().push_cswap(0, 2, 1).unwrap().build();

    assert!(are_graphs_equivalent(&graph, &graph2));
}

#[test]
#[expect(clippy::unwrap_used)]
fn equivalent_ccx_permutations() {
    let graph = GraphBuilder::default().push_ccx(0, 1, 2).unwrap().build();
    let graph2 = GraphBuilder::default().push_ccx(1, 0, 2).unwrap().build();

    assert!(are_graphs_equivalent(&graph, &graph2));
}

#[test]
#[expect(clippy::unwrap_used)]
fn equivalent_ccz_permutations() {
    let graph = GraphBuilder::default().push_ccz(0, 1, 2).unwrap().build();
    let graph2 = GraphBuilder::default().push_ccz(0, 2, 1).unwrap().build();
    let graph3 = GraphBuilder::default().push_ccz(1, 0, 2).unwrap().build();
    let graph4 = GraphBuilder::default().push_ccz(1, 2, 0).unwrap().build();
    let graph5 = GraphBuilder::default().push_ccz(2, 0, 1).unwrap().build();
    let graph6 = GraphBuilder::default().push_ccz(2, 1, 0).unwrap().build();

    assert!(are_graphs_equivalent(&graph, &graph2));
    assert!(are_graphs_equivalent(&graph, &graph3));
    assert!(are_graphs_equivalent(&graph, &graph4));
    assert!(are_graphs_equivalent(&graph, &graph5));
    assert!(are_graphs_equivalent(&graph, &graph6));

    assert!(are_graphs_equivalent(&graph2, &graph3));
    assert!(are_graphs_equivalent(&graph2, &graph4));
    assert!(are_graphs_equivalent(&graph2, &graph5));
    assert!(are_graphs_equivalent(&graph2, &graph6));

    assert!(are_graphs_equivalent(&graph3, &graph4));
    assert!(are_graphs_equivalent(&graph3, &graph5));
    assert!(are_graphs_equivalent(&graph3, &graph6));

    assert!(are_graphs_equivalent(&graph4, &graph5));
    assert!(are_graphs_equivalent(&graph4, &graph6));

    assert!(are_graphs_equivalent(&graph5, &graph6));
}
