from enum import Enum
from qiskit import QuantumCircuit

class GraphvizFormat(Enum):
    """Output format for visual quantum graph representations powered by Graphviz."""

    GV = 0
    """The standard Graphviz `dot` format, for use with other tools that can render Graphviz graphs."""
    PNG = 1
    """The PNG image format, for rasterized output compatible with many programs."""
    SVG = 2
    """The SVG image format, for lightweight vector graphics output."""

def display_graph(circuit: QuantumCircuit) -> str:
    """
    Convert a Qiskit `QuantumCircuit` into a string representing its contents.

    The contents are shown as a list of nodes and edges.
    The output is as fancy as possible, intended for human consumption.
    """
    ...

def display_grid(circuit: QuantumCircuit) -> str:
    """
    Convert a Qiskit `QuantumCircuit` into a 2D grid representing its contents.

    Every row represents a qubit, and every column is specific instant in time, ordered from left to right.
    Multi-qubit gates are represented by multiple rows, one for each qubit.
    The output is as fancy as possible, intended for human consumption.
    """
    ...

def display_matrix(circuit: QuantumCircuit) -> str:
    """
    Convert a Qiskit `QuantumCircuit` into a 2D unitary matrix representing its contents.

    Bra-ket notation is used. Rows represent bras, and columns represent kets.
    The output is as fancy as possible, intended for human consumption.
    """
    ...

def to_graphviz(circuit: QuantumCircuit, format: GraphvizFormat) -> bytes:
    """
    Convert a Qiskit `QuantumCircuit` into a iterable bytes containing a Graphviz graph representation of its contents.

    This creates an easy visual representation of the graph's internal structure.
    All the nodes are positioned similar to how they would be in a circuit diagram.
    It offers many output formats for different use cases.
    """
    ...

def simplify(circuit: QuantumCircuit, iterations: int) -> QuantumCircuit:
    """
    Simplify a Qiskit `QuantumCircuit` into an equivalent circuit.

    Better results may be found by increasing the number of iterations and making the simplification run longer, but there are no guarantees.
    The original circuit is not modified.
    """
    ...
