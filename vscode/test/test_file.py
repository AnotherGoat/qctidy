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


def build_desambiguation_circuit():
    circuit = QuantumCircuit(2, 2)

    circuit.x(0)

    return circuit

def build_rotation_gates_circuit():
    rotations = QuantumCircuit(4, 4)

    rotations.p(numpy.pi / 4, 0)
    rotations.rx(numpy.pi / 8, 1)
    rotations.ry(2 * numpy.pi / 3, 2)
    rotations.rz(-numpy.pi / 2, 3)

    return rotations


def build_append_circuit():
    append = QuantumCircuit(4, 4)

    # sqrt(Y) and general unitary have no named method
    append.append(YGate().power(1 / 2), [1])
    append.u(numpy.pi / 2, numpy.pi / 3, numpy.pi, 2)

    return append


def build_two_qubit_gates_circuit():
    two = QuantumCircuit(4, 4)

    two.swap(0, 3)
    two.ch(1, 2)
    two.cx(2, 0)
    two.cy(3, 1)
    two.cz(0, 1)
    two.cp(numpy.pi / 4, 2, 3)

    return two


def build_three_qubit_gates_circuit():
    three = QuantumCircuit(4, 4)

    three.cswap(0, 1, 2)
    three.ccx(1, 2, 3)
    three.ccz(2, 3, 0)

    return three


def build_multiline_circuit():
    multiline = QuantumCircuit(4, 4)

    # fmt: off
    multiline.ccx(
        0,
        1,
        2
    )
    # fmt: on

    return multiline


def build_measurements_circuit():
    measured1 = QuantumCircuit(4, 4)

    measured1.measure(0, 2)
    measured1.measure(1, 0)
    measured1.measure(2, 3)
    measured1.measure(3, 1)

    return measured1


def build_measure_all_circuit():
    measured2 = QuantumCircuit(3)

    measured2.h(0)
    measured2.cx(0, 1)
    measured2.measure_all()

    return measured2


def build_repeated_measure_all_circuit():
    measured3 = QuantumCircuit(3)

    measured3.h(0)
    measured3.measure_all()
    measured3.measure_all()

    return measured3


def build_constant_sized_circuit():
    qubit_count = 3
    sized = QuantumCircuit(qubit_count)

    sized.h(0)
    sized.cx(0, 1)

    return sized


def build_empty_circuit():
    empty = QuantumCircuit(0)

    return empty


def build_fixable_circuit():
    fixable = QuantumCircuit(3)

    # R001: identity gate
    fixable.id(0)

    # R002: two Hadamards
    fixable.h(1)
    fixable.h(1)

    # P001: two S gates
    fixable.s(0)
    fixable.s(0)

    # R003: two X gates
    fixable.x(2)
    fixable.x(2)

    # R006: two CX gates
    fixable.cx(0, 1)
    fixable.cx(0, 1)
    fixable.cx(0, 1)

    # G001: 3 CX -> SWAP
    fixable.cx(1, 0)
    fixable.cx(0, 1)

    # B001: H X H
    fixable.h(2)
    fixable.x(2)
    fixable.h(2)

    # X003: triangular CX chain
    fixable.cx(1, 2)
    fixable.cx(0, 1)
    fixable.cx(0, 2)

    # C003: angle normalization
    fixable.rz(-numpy.pi / 2, 0)

    return fixable
