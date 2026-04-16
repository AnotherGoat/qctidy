use GateType::*;
use qsimplify::{
    domain::{GateType, GraphBuilder, QuantumGraph},
    dto::gate::GateOperation,
};

/// Convert a list of gate operations into a `QuantumGraph`.
pub fn build_graph_from_operations(operations: Vec<GateOperation>) -> QuantumGraph {
    let mut builder = GraphBuilder::new();

    for operation in operations {
        match operation.r#type {
            H => builder.push_h(operation.qubits[0]),
            X => builder.push_x(operation.qubits[0]),
            CX => builder.push_cx(operation.qubits[0], operation.qubits[1]),
            MEASURE => builder.push_measure(operation.qubits[0], operation.bits[0]),
            _ => panic!("Unsupported gate (for now)"),
        };
    }

    builder.build()
}
