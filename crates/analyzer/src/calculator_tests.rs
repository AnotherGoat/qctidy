use qctidy::math::are_floats_equal;
use qctidy::number::truncate_usize_to_isize;
use qctidy::{Graph, GraphBuilder};
use qctidy_ports::{AnalysisMetrics, DeltaAnalysisMetrics};

use crate::calculator::{calculate_detailed_metrics, calculate_metrics, compare_metrics};

#[test]
fn calculates_expected_metrics() {
    let mut builder = GraphBuilder::new(2);
    builder.push_h(0);
    builder.push_cx(0, 1).unwrap();
    let graph = builder.into_graph();

    let metrics = calculate_metrics(&graph);

    assert_eq!(metrics.qubit_count, 2);
    assert_eq!(metrics.bit_count, 0);
    assert_eq!(metrics.gate_count, 2);
    assert_eq!(metrics.hadamard_count, 1);
    assert_eq!(metrics.cx_count, 1);
    assert_eq!(metrics.x_count, 0);
    assert_eq!(metrics.phase_count, 0);
    assert_eq!(metrics.t_count, 0);
    assert_eq!(metrics.ccx_count, 0);
    assert_eq!(metrics.single_gate_count, 1);
    assert_eq!(metrics.controlled_gate_count, 1);
}

#[test]
fn empty_graph_does_not_panic() {
    let graph = Graph::new(0);
    let detailed = calculate_detailed_metrics(&graph);
    assert!(are_floats_equal(detailed.average_density, 0.0));
    assert!(are_floats_equal(detailed.single_gate_percent, 0.0));
    assert!(are_floats_equal(detailed.cx_qubit_percent, 0.0));
}

#[test]
fn excludes_measurements_from_gate_counts() {
    let mut builder = GraphBuilder::new(2);
    builder.push_h(0);
    builder.push_measure(0, 0);
    let graph = builder.into_graph();

    let metrics = calculate_metrics(&graph);

    assert_eq!(metrics.operation_count, 2);
    assert_eq!(metrics.gate_count, 1);
    assert_eq!(metrics.measurement_count, 1);
    assert_eq!(metrics.single_gate_count, 1);
}

#[test]
fn calculates_phase_t_ccx_and_bit_counts() {
    let mut builder = GraphBuilder::new(3);
    builder.push_p(0.5, 0).unwrap();
    builder.push_t(1);
    builder.push_tdg(2);
    builder.push_ccx(0, 1, 2).unwrap();
    builder.push_measure(0, 0);
    let graph = builder.into_graph();

    let metrics = calculate_metrics(&graph);

    assert_eq!(metrics.phase_count, 1);
    assert_eq!(metrics.t_count, 2);
    assert_eq!(metrics.ccx_count, 1);
    assert_eq!(metrics.bit_count, 1);
}

fn difference(old: usize, new: usize) -> isize {
    truncate_usize_to_isize(new) - truncate_usize_to_isize(old)
}

fn comparison_pair() -> (Graph, Graph) {
    let mut old = GraphBuilder::new(2);
    old.push_h(0);
    old.push_x(1);

    let mut new = GraphBuilder::new(3);
    new.push_h(0);
    new.push_x(1);
    new.push_y(2);
    new.push_z(0);
    new.push_p(0.5, 1).unwrap();
    new.push_rx(0.5, 2).unwrap();
    new.push_s(1);
    new.push_t(2);
    new.push_tdg(0);
    new.push_swap(0, 1).unwrap();
    new.push_cx(0, 1).unwrap();
    new.push_ccx(0, 1, 2).unwrap();
    new.push_measure(2, 0);

    (old.into_graph(), new.into_graph())
}

fn comparison_delta() -> (AnalysisMetrics, AnalysisMetrics, DeltaAnalysisMetrics) {
    let (old, new) = comparison_pair();
    let old_metrics = calculate_metrics(&old);
    let new_metrics = calculate_metrics(&new);
    let delta = compare_metrics(&old, &new);
    (old_metrics, new_metrics, delta)
}

#[test]
fn compare_metrics_reports_size_deltas() {
    let (old_metrics, new_metrics, delta) = comparison_delta();

    assert_eq!(
        delta.qubit_count,
        difference(old_metrics.qubit_count, new_metrics.qubit_count)
    );
    assert_eq!(
        delta.bit_count,
        difference(old_metrics.bit_count, new_metrics.bit_count)
    );
    assert_eq!(
        delta.time_step_count,
        difference(old_metrics.time_step_count, new_metrics.time_step_count)
    );
}

#[test]
fn compare_metrics_reports_single_qubit_deltas() {
    let (old_metrics, new_metrics, delta) = comparison_delta();

    assert_eq!(
        delta.x_count,
        difference(old_metrics.x_count, new_metrics.x_count)
    );
    assert_eq!(
        delta.y_count,
        difference(old_metrics.y_count, new_metrics.y_count)
    );
    assert_eq!(
        delta.z_count,
        difference(old_metrics.z_count, new_metrics.z_count)
    );
    assert_eq!(
        delta.pauli_count,
        difference(old_metrics.pauli_count, new_metrics.pauli_count)
    );
    assert_eq!(
        delta.hadamard_count,
        difference(old_metrics.hadamard_count, new_metrics.hadamard_count)
    );
    assert_eq!(
        delta.rotation_count,
        difference(old_metrics.rotation_count, new_metrics.rotation_count)
    );
    assert_eq!(
        delta.phase_count,
        difference(old_metrics.phase_count, new_metrics.phase_count)
    );
    assert_eq!(
        delta.square_root_count,
        difference(old_metrics.square_root_count, new_metrics.square_root_count)
    );
    assert_eq!(
        delta.t_count,
        difference(old_metrics.t_count, new_metrics.t_count)
    );
}

#[test]
fn compare_metrics_reports_multi_qubit_deltas() {
    let (old_metrics, new_metrics, delta) = comparison_delta();

    assert_eq!(
        delta.swap_count,
        difference(old_metrics.swap_count, new_metrics.swap_count)
    );
    assert_eq!(
        delta.cx_count,
        difference(old_metrics.cx_count, new_metrics.cx_count)
    );
    assert_eq!(
        delta.ccx_count,
        difference(old_metrics.ccx_count, new_metrics.ccx_count)
    );
}

#[test]
fn compare_metrics_reports_operation_deltas() {
    let (old_metrics, new_metrics, delta) = comparison_delta();

    assert_eq!(
        delta.operation_count,
        difference(old_metrics.operation_count, new_metrics.operation_count)
    );
    assert_eq!(
        delta.gate_count,
        difference(old_metrics.gate_count, new_metrics.gate_count)
    );
    assert_eq!(
        delta.measurement_count,
        difference(old_metrics.measurement_count, new_metrics.measurement_count)
    );
    assert_eq!(
        delta.single_gate_count,
        difference(old_metrics.single_gate_count, new_metrics.single_gate_count)
    );
    assert_eq!(
        delta.controlled_gate_count,
        difference(
            old_metrics.controlled_gate_count,
            new_metrics.controlled_gate_count
        )
    );
}

#[test]
fn compare_metrics_reports_measurement_and_gate_type_deltas() {
    let (old_metrics, new_metrics, delta) = comparison_delta();

    assert_eq!(
        delta.measured_qubit_count,
        difference(
            old_metrics.measured_qubit_count,
            new_metrics.measured_qubit_count
        )
    );
    assert_eq!(
        delta.unmeasured_qubit_count,
        difference(
            old_metrics.unmeasured_qubit_count,
            new_metrics.unmeasured_qubit_count
        )
    );
    assert_eq!(
        delta.gate_types_count,
        difference(old_metrics.gate_types_count, new_metrics.gate_types_count)
    );
}
