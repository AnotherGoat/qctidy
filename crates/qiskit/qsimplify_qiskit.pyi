from enum import Enum
from qiskit import QuantumCircuit

class DisplayFormat(Enum):
    """Format for plain text quantum graph representations."""

    GRAPH = 0
    """
    The contents are shown as a list of nodes and edges.

    DiracFormat is ignored.
    """
    GRID = 1
    """
    Represents the graph as a 2D grid.

    Every row represents a qubit, and every column is specific instant in time, ordered from left to right.
    Multi-qubit gates are represented by multiple rows, one for each qubit.
    DiracFormat is ignored.
    """
    MATRIX = 2
    """
    Represents the graph as a 2D unitary matrix.

    Bra-ket notation is used. Rows represent bras, and columns represent kets.
    PiFormat is ignored.
    """

class PiFormat(Enum):
    """Format to used to display the pi constant in angles."""
    LOWERCASE = 0
    """Display the pi constant as `pi`, ASCII-compatible."""
    UPPERCASE = 1
    """Display the pi constant as `PI`, ASCII-compatible."""
    FANCY = 2
    """Display the pi constant as `π`."""

class DiracFormat(Enum):
    """Format used to display Dirac bra-ket notation."""
    ASCII = 0
    """Display bras (rows) as `<10|` and kets (columns) as `|10>`, ASCII-compatible."""
    FANCY = 1
    """Display bras (rows) as `⟨10∣` and kets (columns) as `∣10⟩`."""
    NONE = 2
    """Don't use bra-ket notation, just keep the indices."""

class PresentationFormat(Enum):
    """Output format for visual quantum graph representations."""

    GRAPHVIZ_GV = 0
    """
    The standard Graphviz `dot` format, for use with other tools that can render Graphviz graphs.

    The DPI parameter is ignored.
    """
    GRAPHVIZ_PNG = 1
    """The PNG image format, for rasterized output compatible with many programs."""
    GRAPHVIZ_SVG = 2
    """The SVG image format, for lightweight vector graphics output.

    The DPI parameter is ignored.
    """

class ConversionFormat(Enum):
    """Output format for parsing and serialization of quantum circuits."""

    JSON = 0
    """Convert to and from a JSON string, which can be pretty-printed if desired.

    The indentation parameter is used to determine the indentation level, only for pretty serialization.
    """
    MESSAGE_PACK = 1
    """Convert to and from a MessagePack binary blob.

    The format works similarly to JSON, but encodes data in a more compact manner.
    The prettify and indentation parameters are ignored.
    """
    BINARY = 2
    """Convert to and from a binary blob.

    The prettify and indentation parameters are ignored.
    """
    BASE64 = 3
    """Convert to and from a base64-encoded binary blob.

    The prettify and indentation parameters are ignored.
    """

class CodeGenerationTarget(Enum):
    """Output format for generated code."""
    QISKIT = 0
    """Generate Qiskit Python code, which uses QuantumCircuit to build circuits."""

def display(
    circuit: QuantumCircuit,
    format: DisplayFormat,
    pi_format: PiFormat | None = None,
    dirac_format: DiracFormat | None = None,
) -> str:
    """Convert a Qiskit `QuantumCircuit` into a string representing its contents.

    By default, the output will be as fancy as possible, intended for human consumption.
    """

def present(circuit: QuantumCircuit, format: PresentationFormat, dpi: int | None = None) -> bytes:
    """
    Convert a Qiskit `QuantumCircuit` into a iterable bytes containing a Graphviz graph representation of its contents.

    This creates an easy visual representation of the graph's internal structure.
    All the nodes are positioned similar to how they would be in a circuit diagram.
    It offers many output formats for different use cases.
    """

def simplify(circuit: QuantumCircuit, iterations: int) -> QuantumCircuit:
    """
    Simplify a Qiskit `QuantumCircuit` into an equivalent circuit.

    Better results may be found by increasing the number of iterations and making the simplification run longer, but there are no guarantees.
    The original circuit is not modified.
    """

def parse(input: bytes, format: ConversionFormat) -> QuantumCircuit:
    """Convert bytes in the specified format into a Qiskit `QuantumCircuit`."""

def serialize(circuit: QuantumCircuit, format: ConversionFormat, prettify: bool | None = None, indentation: int | None = None) -> bytes:
    """Convert a Qiskit `QuantumCircuit` graph representation into bytes in the specified format."""

def generate_code(circuit: QuantumCircuit, target: CodeGenerationTarget, circuit_name: str | None = None) -> str:
    """Generate to build the provided Qiskit `QuantumCircuit`."""
