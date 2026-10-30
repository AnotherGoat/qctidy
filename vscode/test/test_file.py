import numpy
from qiskit import QuantumCircuit
from qiskit.circuit.library.standard_gates import YGate


def build_single_qubit_gates_circuit():
    circuit = QuantumCircuit(4, 4)

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

    return circuit


def build_rotation_gates_circuit():
    circuit = QuantumCircuit(4, 4)

    circuit.p(numpy.pi / 4, 0)
    circuit.rx(numpy.pi / 8, 1)
    circuit.ry(2 * numpy.pi / 3, 2)
    circuit.rz(-numpy.pi / 2, 3)

    return circuit


def build_append_circuit():
    circuit = QuantumCircuit(4, 4)

    # sqrt(Y) and general unitary have no named method
    circuit.append(YGate().power(1 / 2), [1])
    circuit.u(numpy.pi / 2, numpy.pi / 3, numpy.pi, 2)

    return circuit


def build_two_qubit_gates_circuit():
    circuit = QuantumCircuit(4, 4)

    circuit.swap(0, 3)
    circuit.ch(1, 2)
    circuit.cx(2, 0)
    circuit.cy(3, 1)
    circuit.cz(0, 1)
    circuit.cp(numpy.pi / 4, 2, 3)

    return circuit


def build_three_qubit_gates_circuit():
    circuit = QuantumCircuit(4, 4)

    circuit.cswap(0, 1, 2)
    circuit.ccx(1, 2, 3)
    circuit.ccz(2, 3, 0)

    return circuit


def build_multiline_circuit():
    circuit = QuantumCircuit(4, 4)

    # fmt: off
    circuit.ccx(
        0,
        1,
        2
    )
    # fmt: on

    return circuit


def build_measurements_circuit():
    circuit = QuantumCircuit(4, 4)

    circuit.measure(0, 2)
    circuit.measure(1, 0)
    circuit.measure(2, 3)
    circuit.measure(3, 1)

    return circuit


def build_measure_all_circuit():
    circuit = QuantumCircuit(3)

    circuit.h(0)
    circuit.cx(0, 1)
    circuit.measure_all()

    return circuit


def build_repeated_measure_all_circuit():
    circuit = QuantumCircuit(3)

    circuit.h(0)
    circuit.measure_all()
    circuit.measure_all()

    return circuit


def build_constant_sized_circuit():
    qubit_count = 3
    circuit = QuantumCircuit(qubit_count)

    circuit.h(0)
    circuit.cx(0, 1)

    return circuit


def build_empty_circuit():
    circuit = QuantumCircuit(0)

    return circuit


def build_fixable_circuit():
    circuit = QuantumCircuit(3)

    circuit.id(0)       # R001: identity gate
    circuit.h(1)        # R002: two Hadamards
    circuit.h(1)
    circuit.s(0)        # P001: two S gates
    circuit.s(0)
    circuit.x(2)        # R003: two X gates
    circuit.x(2)
    circuit.cx(0, 1)    # R006: two CX gates
    circuit.cx(0, 1)
    circuit.cx(0, 1)    # G001: 3 CX -> SWAP
    circuit.cx(1, 0)
    circuit.cx(0, 1)
    circuit.h(2)        # B001: H X H
    circuit.x(2)
    circuit.h(2)
    circuit.cx(1, 2)    # X003: triangular CX chain
    circuit.cx(0, 1)
    circuit.cx(0, 2)
    circuit.rz(-numpy.pi / 2, 0)  # C003: angle normalization

    return circuit

