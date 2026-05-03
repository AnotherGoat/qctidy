# Changelog

## v0.1.0 (pre-release)

- `library`: Represent quantum circuits as a hybrid directed graph / 2D matrix.
- `library`: Expose a Rust fluent `GraphBuilder` API.
- `library`: Design and validate a quantum graph by using schemas.
- `library`: Display a quantum graph as a list of nodes and edges or as a less detailed 2D matrix.
- `library`: Calculate the unitary matrix of a quantum graph.
- `library`: Display a quantum graph as a unitary matrix with Dirac bra-ket notation.
- `presenter`: Allow saving a quantum graph as a graphviz graph in GV, PNG and SVG formats.
- `converter`: Parse and serialize quantum graphs in JSON and MessagePack formats.
- `facade`: Create a DTO that stores a list of quantum gate operations in insertion order.
- `facade`: Expose a use case for displaying the provided circuit as a graph.
- `qiskit`: Create bindings that receive a Qiskit `QuantumCircuit` as input.
- `qiskit`: Make it possible to return a Qiskit `QuantumCircuit` as output.
