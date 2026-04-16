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
            ID => builder.push_id(operation.qubits[0]),
            H => builder.push_h(operation.qubits[0]),
            X => builder.push_x(operation.qubits[0]),
            Y => builder.push_y(operation.qubits[0]),
            Z => builder.push_z(operation.qubits[0]),
            P => builder.push_p(operation.parameters[0], operation.qubits[0]),
            RX => builder.push_rx(operation.parameters[0], operation.qubits[0]),
            RY => builder.push_ry(operation.parameters[0], operation.qubits[0]),
            RZ => builder.push_rz(operation.parameters[0], operation.qubits[0]),
            S => builder.push_s(operation.qubits[0]),
            SDG => builder.push_sdg(operation.qubits[0]),
            SX => builder.push_sx(operation.qubits[0]),
            SY => builder.push_sy(operation.qubits[0]),
            T => builder.push_t(operation.qubits[0]),
            TDG => builder.push_tdg(operation.qubits[0]),
            Measure => builder.push_measure(operation.qubits[0], operation.bits[0]),
            Swap => builder.push_swap(operation.qubits[0], operation.qubits[1]),
            CH => builder.push_ch(operation.qubits[0], operation.qubits[1]),
            CX => builder.push_cx(operation.qubits[0], operation.qubits[1]),
            CY => builder.push_cy(operation.qubits[0], operation.qubits[1]),
            CZ => builder.push_cz(operation.qubits[0], operation.qubits[1]),
            CP => builder.push_cp(
                operation.parameters[0],
                operation.qubits[0],
                operation.qubits[1],
            ),
            CSwap => builder.push_cswap(
                operation.qubits[0],
                operation.qubits[1],
                operation.qubits[2],
            ),
            CCX => builder.push_ccx(
                operation.qubits[0],
                operation.qubits[1],
                operation.qubits[2],
            ),
            CCZ => builder.push_ccz(
                operation.qubits[0],
                operation.qubits[1],
                operation.qubits[2],
            ),
        };
    }

    builder.build()
}
