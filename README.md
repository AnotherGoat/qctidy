# QSimplify

A toolset for quantum circuit simplification written in Rust, with an emphasis on code quality and understandability.

It started as an enhanced port of a previousprototype that was written in Python. The new version uses Rust to provide these benefits:

- Better performance and memory usage, because it's compiled to native code and uses zero-cost abstractions.
- More reliability due to Rust's stronger type system and ownership model.
- More robust error handling and null-safety due to Rust's `Result` and `Option` types.
- Occasional code simplification thanks to Rust's macro system.
- The same interface as before can be kept for Python users by providing PyO3 bindings.

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Project Structure](#project-structure)
- [Useful Commands](#useful-commands)
  - [Rust Library](#rust-library)
  - [Python Bindings for Qiskit](#python-bindings-for-qiskit)
  - [Supported Gates](#supported-gates)
- [Contributing](#contributing)
  - [Coding Conventions](#coding-conventions)
- [License](#license)

## Features

- **Graph-based circuit representation**: quantum circuits are modelled as a hybrid directed graph / 2D matrix (`QuantumGraph`), where each qubit is a row and each "time step" is a column.
  - This structure allows easy analysis and manipulation.
  - The directed graph is used to store semantic relationships between nodes (e.g., control or target qubits).
  - The 2D matrix is used to speed up element access by position

- **Rich gate support**: The most commonly used gates are availabe, including:
  - Single-qubit gates (H, X, Y, Z, P, RX, RY, RZ, S, SDG, SX, SY, T, TDG, Measure).
  - Two-qubit gates (SWAP, CH, CX, CY, CZ, CP).
  - Three-qubit gates (CSwap / Fredkin, CCX / Toffoli, CCZ).

- **Fluent builder API**: `GraphBuilder` provides a safe, chainable interface for constructing circuits without touching the graph internals directly.
  - Any circuit built by using the builder is guaranteed to be internally consistent.

- **Python bindings**: They are exposed as a Python extension module via [PyO3](https://pyo3.rs). So far there are bindings for:
  - [Qiskit](https://www.ibm.com/quantum/qiskit), in [crates/qiskit](crates/qiskit).

- **CLI**: Command-line interface for loading and processing circuits.

- **Serialization**: JSON-based I/O for converting circuits to various formats and saving or loading them from disk.

## Installation

TODO.

## Project Structure

The project is composed of various Rust crates, which focus on making it modular and extensible. A facade exposes all the common use cases so that client implementations don't have to worry about the internal details.

```plain
.                       # Project root
├── Cargo.lock          # Global lockfile
├── Cargo.toml          # Cargo workspace root, each crate also has its own Cargo.toml
├── CONVENTIONS.md      # Repository guidelines and code conventions
├── prek.toml           # Configuration for pre-commit hooks
├── README.md           # Project description (this file)
└── crates              # Workspace member crates
    ├── cli             # Command-line interface (binary)
    ├── codegen         # Code generator (library)
    ├── converter       # Circuit parser and serializer for various formats (library)
    ├── estimator       # Circuit execution time and cost estimation (library)
    ├── facade          # Facade for client implementation (library)
    ├── library         # The core of QSimplify (library)
    ├── presenter       # Circuit and graph visualization (library)
    ├── qiskit          # Python + Qiskit bindings via PyO3 (library)
    ├── server          # Server implementation as a REST API (binary)
    └── storage         # Circuit and simplification rule storage (library)
```

Note: Use `tree --gitignore -L2 --filesfirst` to see the structure.

## Useful Commands

These Cargo commands can be run from the root of the repository.

| Command                             | Purpose                                               |
| ----------------------------------- | ----------------------------------------------------- |
| `cargo fmt`                         | Auto-format all source files.                         |
| `cargo clippy --workspace`          | Lint code in all the crates.                          |
| `cargo check --workspace`           | Check that all the crates compile.                    |
| `cargo build --workspace`           | Compile a debug build of all the crates.              |
| `cargo build --release --workspace` | Compile an optimized release build of all the crates. |
| `cargo test --workspace`            | Run all unit and integration tests.                   |
| `cargo test -p <crate>`             | Run tests for a single crate.                         |

For more specific tasks, these commands can be used:

| Command | Purpose |
| ----------------------------------- | ----------------------------------------------------- | `prek run --all-files --show-diff-on-failure` | Manually run pre-commit hooks. |

## Usage

### Rust Library

TODO.

### Python Bindings for Qiskit

TODO.

### Supported Gates

TODO.

## Contributing

1. Fork the repository and create a feature branch.
2. Follow the coding conventions below.
3. Submit a pull request.
4. Wait for all the checks to pass and for the changes to be approved.

### Coding Conventions

Check the [CONVENTIONS.md](CONVENTIONS.md) file for a detailed list of repository and coding conventions.

## License

TODO.
