use qsimplify::{GateType, GraphBuilder};

use crate::models::BackendProfile;
use crate::time_estimator::{estimate_execution_time, get_gate_cost};

#[test]
fn gets_gate_cost_from_backend_profile() {
    let profile = BackendProfile::default();

    assert_eq!(get_gate_cost(GateType::H, &profile), 1.0);
    assert_eq!(get_gate_cost(GateType::CX, &profile), 15.0);
    assert_eq!(get_gate_cost(GateType::CCX, &profile), 80.0);
}

#[test]
fn estimates_execution_time_with_parallel_gates() {
    let profile = BackendProfile::default();
    let mut builder = GraphBuilder::new(3);
    builder.push_h(0);
    builder.push_h(1);
    builder.push_cx(0, 1).unwrap();

    let graph = builder.build();
    let shots = 10;

    let time = estimate_execution_time(&graph, shots, &profile);

    assert_eq!(time, 1160.0);
}
