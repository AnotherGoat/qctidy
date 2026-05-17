use qsimplify::GraphBuilder;
use std::f64::consts::PI;

use crate::qiskit;

#[test]
fn generate_empty_circuit() {
    let graph = GraphBuilder::default().build();
    let actual = qiskit::generate(&graph, "circuit");

    let expected = "\
from qiskit import QuantumCircuit

circuit = QuantumCircuit()";

    assert_eq!(actual, expected);
}

#[test]
fn generate_single_qubit_circuit() {
    let graph = GraphBuilder::default()
        .push_h(0)
        .push_x(0)
        .push_y(0)
        .push_z(0)
        .push_t(0)
        .build();
    let actual = qiskit::generate(&graph, "circuit");

    let expected = "\
from qiskit import QuantumCircuit

circuit = QuantumCircuit(1)
circuit.h(0)
circuit.x(0)
circuit.y(0)
circuit.z(0)
circuit.t(0)";

    assert_eq!(actual, expected);
}

#[test]
fn generate_circuit_with_measurements() {
    let graph = GraphBuilder::default().push_h(0).push_measure(0, 0).build();
    let actual = qiskit::generate(&graph, "circuit");

    let expected = "\
from qiskit import QuantumCircuit

circuit = QuantumCircuit(1, 1)
circuit.h(0)
circuit.measure(0, 0)";

    assert_eq!(actual, expected);
}

#[test]
#[expect(clippy::unwrap_used)]
fn generate_circuit_with_rotations() {
    let graph = GraphBuilder::default()
        .push_rx(PI, 0)
        .unwrap()
        .push_ry(PI / 2.0, 0)
        .unwrap()
        .push_rz(2.0 * PI / 3.0, 0)
        .unwrap()
        .push_p(5.0 * PI / 4.0, 0)
        .unwrap()
        .build();
    let actual = qiskit::generate(&graph, "circuit");

    let expected = "\
import numpy
from qiskit import QuantumCircuit

circuit = QuantumCircuit(1)
circuit.rx(numpy.pi, 0)
circuit.ry(numpy.pi / 2, 0)
circuit.rz(2 * numpy.pi / 3, 0)
circuit.p(5 * numpy.pi / 4, 0)";

    assert_eq!(actual, expected);
}

#[test]
#[expect(clippy::unwrap_used)]
fn generate_circuit_with_zero_rotations() {
    let graph = GraphBuilder::default()
        .push_p(0.0, 0)
        .unwrap()
        .push_p(1e-8, 0)
        .unwrap()
        .build();
    let actual = qiskit::generate(&graph, "circuit");

    let expected = "\
from qiskit import QuantumCircuit

circuit = QuantumCircuit(1)
circuit.p(0, 0)
circuit.p(0, 0)";

    assert_eq!(actual, expected,);
}

#[test]
#[expect(clippy::unwrap_used)]
fn generate_circuit_with_uncommon_rotations() {
    let graph = GraphBuilder::default()
        .push_rx(0.222, 0)
        .unwrap()
        .push_ry(1.333, 0)
        .unwrap()
        .push_rz(2.444, 0)
        .unwrap()
        .push_p(3.555, 0)
        .unwrap()
        .build();
    let actual = qiskit::generate(&graph, "circuit");

    let expected = "\
from qiskit import QuantumCircuit

circuit = QuantumCircuit(1)
circuit.rx(0.222, 0)
circuit.ry(1.333, 0)
circuit.rz(2.444, 0)
circuit.p(3.555, 0)";

    assert_eq!(actual, expected);
}

#[test]
fn generate_circuit_with_sy_gate() {
    let graph = GraphBuilder::default().push_sy(0).build();
    let actual = qiskit::generate(&graph, "circuit");

    let expected = "\
from qiskit import QuantumCircuit
from qiskit.circuit.library.standard_gates import YGate

circuit = QuantumCircuit(1)
circuit.append(YGate().power(1 / 2), [0])";

    assert_eq!(actual, expected);
}

#[test]
#[expect(clippy::unwrap_used)]
fn generate_circuit_with_two_qubits() {
    let graph = GraphBuilder::default()
        .push_cx(0, 1)
        .unwrap()
        .push_cy(1, 0)
        .unwrap()
        .build();
    let actual = qiskit::generate(&graph, "circuit");

    let expected = "\
from qiskit import QuantumCircuit

circuit = QuantumCircuit(2)
circuit.cx(0, 1)
circuit.cy(1, 0)";

    assert_eq!(actual, expected);
}

#[test]
fn generate_circuit_orders_qubits() {
    let graph = GraphBuilder::default()
        .push_h(4)
        .push_x(2)
        .push_y(0)
        .push_z(1)
        .push_s(3)
        .build();
    let actual = qiskit::generate(&graph, "circuit");

    let expected = "\
from qiskit import QuantumCircuit

circuit = QuantumCircuit(5)
circuit.y(0)
circuit.z(1)
circuit.x(2)
circuit.s(3)
circuit.h(4)";

    assert_eq!(actual, expected);
}

#[test]
#[expect(clippy::unwrap_used)]
fn generate_circuit_orders_symmetrical_gates() {
    let graph = GraphBuilder::default()
        .push_swap(1, 0)
        .unwrap()
        .push_cz(2, 0)
        .unwrap()
        .push_swap(3, 1)
        .unwrap()
        .push_cz(3, 2)
        .unwrap()
        .build();
    let actual = qiskit::generate(&graph, "circuit");

    let expected = "\
from qiskit import QuantumCircuit

circuit = QuantumCircuit(4)
circuit.swap(0, 1)
circuit.cz(0, 2)
circuit.swap(1, 3)
circuit.cz(2, 3)";

    assert_eq!(actual, expected);
}

#[test]
#[expect(clippy::unwrap_used)]
fn generate_circuit_with_three_qubits() {
    let graph = GraphBuilder::default()
        .push_ccx(0, 2, 1)
        .unwrap()
        .push_cswap(1, 0, 2)
        .unwrap()
        .push_ccz(0, 1, 2)
        .unwrap()
        .build();
    let actual = qiskit::generate(&graph, "circuit");

    let expected = "\
from qiskit import QuantumCircuit

circuit = QuantumCircuit(3)
circuit.ccx(0, 2, 1)
circuit.cswap(1, 0, 2)
circuit.ccz(0, 1, 2)";

    assert_eq!(actual, expected);
}

#[test]
fn generate_circuit_with_multiple_bits() {
    let graph = GraphBuilder::default()
        .push_h(0)
        .push_x(1)
        .push_y(2)
        .push_measure(0, 0)
        .push_measure(1, 1)
        .push_measure(2, 2)
        .build();
    let actual = qiskit::generate(&graph, "circuit");

    let expected = "\
from qiskit import QuantumCircuit

circuit = QuantumCircuit(3, 3)
circuit.h(0)
circuit.x(1)
circuit.y(2)
circuit.measure(0, 0)
circuit.measure(1, 1)
circuit.measure(2, 2)";

    assert_eq!(actual, expected);
}

#[test]
fn generate_circuit_with_flipped_measurements() {
    let graph = GraphBuilder::default()
        .push_h(0)
        .push_z(1)
        .push_t(2)
        .push_measure(0, 1)
        .push_measure(1, 2)
        .push_measure(2, 0)
        .build();
    let actual = qiskit::generate(&graph, "circuit");

    let expected = "\
from qiskit import QuantumCircuit

circuit = QuantumCircuit(3, 3)
circuit.h(0)
circuit.z(1)
circuit.t(2)
circuit.measure(0, 1)
circuit.measure(1, 2)
circuit.measure(2, 0)";

    assert_eq!(actual, expected);
}

#[test]
fn generate_circuit_with_custom_name() {
    let graph = GraphBuilder::default()
        .push_x(0)
        .push_y(0)
        .push_z(0)
        .build();
    let actual = qiskit::generate(&graph, "xyz");

    let expected = "\
from qiskit import QuantumCircuit

xyz = QuantumCircuit(1)
xyz.x(0)
xyz.y(0)
xyz.z(0)";

    assert_eq!(actual, expected);
}
