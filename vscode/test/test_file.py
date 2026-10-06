"""Sample circuit that uses every gate supported by the QCTidy Rust crates.

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
