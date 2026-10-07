use std::f64::consts::PI;

use qctidy::{GateOperation, GateType};
use qctidy_ports::{ParsedCircuit, PythonPort, Scope, ScopeChild, ScopeType};

use crate::PythonAdapter;

const ALL_GATES: &str = r#""""Sample circuit that uses every gate supported by the QCTidy Rust crates.

The gate list mirrors `GateType` in
`crates/library/src/domain/graph/gate_type.rs`.
"""

import numpy
from qiskit import QuantumCircuit
from qiskit.circuit.library.standard_gates import YGate


def build_all_gates_circuit():
    circuit = QuantumCircuit(4, 4)

    # Single-qubit gates without parameters
    circuit.id(0)
    circuit.h(1)
    circuit.x(2)
    circuit.y(3)
    circuit.z(2)
    circuit.s(1)
    circuit.sdg(3)
    circuit.sx(0)
    circuit.t(2)
    circuit.tdg(1)

    # Single-qubit rotation and phase gates
    circuit.p(numpy.pi / 4, 0)
    circuit.rx(numpy.pi / 8, 1)
    circuit.ry(2 * numpy.pi / 3, 2)
    circuit.rz(-numpy.pi / 2, 3)

    # sqrt(Y) and general unitary have no named method
    circuit.append(YGate().power(1 / 2), [1])
    circuit.u(numpy.pi / 2, numpy.pi / 3, numpy.pi, 2)

    # Two-qubit gates
    circuit.swap(0, 3)
    circuit.ch(1, 2)
    circuit.cx(2, 0)
    circuit.cy(3, 1)
    circuit.cz(0, 1)
    circuit.cp(numpy.pi / 4, 2, 3)

    # Three-qubit gates
    circuit.cswap(0, 1, 2)
    circuit.ccx(1, 2, 3)
    circuit.ccz(2, 3, 0)

    # Line breaks
    # fmt: off
    circuit.ccx(
        0,
        1,
        2
    )
    # fmt: on

    # Measurements
    circuit.measure(0, 2)
    circuit.measure(1, 0)
    circuit.measure(2, 3)
    circuit.measure(3, 1)

    # Another circuit
    another = QuantumCircuit(2)

    another.h(0)
    another.cx(0, 1)

    # Redefinition of the original circuit
    original = circuit.copy()
    circuit = QuantumCircuit(2)

    circuit.cx(0, 1)
    circuit.cz(1, 0)
    circuit.y(1)

    return original


if __name__ == "__main__":
    circuit = build_all_gates_circuit()
    print(circuit.draw())
"#;

fn circuits_in_scope(scope: &Scope) -> Vec<&ParsedCircuit> {
    scope
        .children
        .iter()
        .filter_map(|child| match child {
            ScopeChild::Circuit(circuit) => Some(circuit),
            ScopeChild::Scope(_) => None,
        })
        .collect()
}

#[test]
fn finds_the_function_and_every_circuit_it_defines() {
    let analysis = PythonAdapter.parse(ALL_GATES);

    assert_eq!(
        analysis.module_circuits.len(),
        0,
        "no circuits at module level"
    );
    assert_eq!(analysis.scopes.len(), 1);

    let scope = &analysis.scopes[0];
    assert_eq!(scope.scope_type, ScopeType::Function);
    assert_eq!(scope.name, "build_all_gates_circuit");
    assert_eq!(scope.children.len(), 3);

    let circuits = circuits_in_scope(scope);
    let first = circuits[0];
    assert_eq!(first.name, "circuit");
    assert_eq!(first.qubits, "4");
    assert_eq!(first.clbits, "4");
    assert_eq!(first.gates.len(), 30);
    assert!(first.issues.is_empty());

    let build = first.build.as_ref().unwrap();
    assert_eq!(build.circuit.qubit_count(), 4);
    assert_eq!(build.circuit.operations().len(), 30);
    assert_eq!(build.source_map.len(), 30);

    assert_eq!(circuits[1].name, "another");
    assert_eq!(circuits[1].qubits, "2");
    assert_eq!(circuits[1].clbits, "0");
    assert_eq!(circuits[1].gates.len(), 2);

    assert_eq!(circuits[2].name, "circuit");
    assert_eq!(circuits[2].qubits, "2");
    assert_eq!(circuits[2].gates.len(), 3);
}

#[test]
fn maps_gate_calls_to_canonical_operations() {
    let analysis = PythonAdapter.parse(ALL_GATES);
    let first = circuits_in_scope(&analysis.scopes[0])[0];

    let hadamard = first
        .gates
        .iter()
        .find(|gate| gate.display_name == "h")
        .unwrap();
    assert_eq!(hadamard.operation.unwrap().r#type(), GateType::H);
    assert_eq!(hadamard.operation.unwrap().qubits(), vec![1]);
    assert_eq!(hadamard.description, "q: 1");

    let sqrt_y = first
        .gates
        .iter()
        .find(|gate| gate.display_name == "sqrt(Y)")
        .unwrap();
    assert_eq!(sqrt_y.operation.unwrap().r#type(), GateType::SY);

    let phase = first
        .gates
        .iter()
        .find(|gate| gate.display_name == "p")
        .unwrap();
    assert_eq!(phase.operation.unwrap().r#type(), GateType::P);

    let GateOperation::P { theta, qubit } = phase.operation.unwrap() else {
        panic!("expected a phase gate");
    };
    assert_eq!(qubit, 0);
    assert!((theta - PI / 4.0).abs() < 1e-12);
}

#[test]
fn reports_uncheckable_circuits_as_issues() {
    let source = "qc = QuantumCircuit(2)\nqc.h(0)\nqc.foo(0)\n";
    let analysis = PythonAdapter.parse(source);

    assert_eq!(analysis.module_circuits.len(), 1);

    let circuit = &analysis.module_circuits[0];
    assert_eq!(circuit.issues.len(), 1);
    assert_eq!(circuit.issues[0].message, "unsupported gate: foo");
    assert!(circuit.build.is_none());
}
