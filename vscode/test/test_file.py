"""Sample circuit that uses every gate supported by the QCTidy Rust crates.

The gate list mirrors `GateType` in
`crates/library/src/domain/graph/gate_type.rs`.
"""

import numpy
from qiskit import QuantumCircuit
from qiskit.circuit.library.standard_gates import YGate


def build_all_gates_circuit():
    circuit = QuantumCircuit(3, 3)

    # Single-qubit gates without parameters
    circuit.id(0)
    circuit.h(0)
    circuit.x(0)
    circuit.y(0)
    circuit.z(0)
    circuit.s(0)
    circuit.sdg(0)
    circuit.sx(0)
    circuit.t(0)
    circuit.tdg(0)

    # Single-qubit rotation and phase gates
    circuit.p(numpy.pi / 4, 0)
    circuit.rx(numpy.pi / 4, 0)
    circuit.ry(numpy.pi / 4, 0)
    circuit.rz(numpy.pi / 4, 0)

    # sqrt(Y) and general unitary has no named method
    circuit.append(YGate().power(1 / 2), [0])
    circuit.u(numpy.pi / 2, 0, numpy.pi, 0)

    # Two-qubit gates
    circuit.swap(0, 1)
    circuit.ch(0, 1)
    circuit.cx(0, 1)
    circuit.cy(0, 1)
    circuit.cz(0, 1)
    circuit.cp(numpy.pi / 4, 0, 1)

    # Three-qubit gates
    circuit.cswap(0, 1, 2)
    circuit.ccx(0, 1, 2)
    circuit.ccz(0, 1, 2)

    # Measurements
    circuit.measure(0, 0)
    circuit.measure(1, 1)
    circuit.measure(2, 2)

    return circuit


if __name__ == "__main__":
    circuit = build_all_gates_circuit()
    print(circuit.draw())
