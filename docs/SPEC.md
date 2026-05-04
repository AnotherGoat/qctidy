# QSimplify Specification

## Table of Contents

- [Functional Requirements](#functional-requirements)
  - [General](#general)
  - [CI/CD](#cicd)
  - [Core Library](#core-library)
  - [Format Conversion](#format-conversion-converter)
  - [Circuit Visualization](#circuit-visualization-presenter)
  - [Code Generation](#code-generation-codegen)
  - [Circuit Analysis](#circuit-analysis-analyzer)
  - [Execution Estimation](#execution-estimation-estimator)
  - [Storage](#storage-storage)
  - [Client Facade and Service Ports](#client-facade-and-service-ports-facade-and-ports)
  - [Python Bindings](#python-bindings-qiskit)
  - [CLI](#cli-cli)
  - [REST API Server](#rest-api-server-server)
- [Non-Functional Requirements](#non-functional-requirements)
  - [Performance](#performance)
  - [Reliability](#reliability)
  - [Usability](#usability)
  - [Maintainability](#maintainability)
  - [Security](#security)

## Functional Requirements

### General

#### Project Information

- [ ] Add a logo for the project
- [ ] Finish all the sections in the project's README
- [ ] Add badges to the project's README
- [ ] Add a license to the project
- [ ] Make the README prettier and more eye-catching
- [ ] Add a video demonstrating the project

#### Feature Management

- [ ] Lock features for each module behind feature gates

### CI/CD

#### Pipeline Creation

- [ ] The code compiles and generates an artifact
- [ ] All the tests pass
- [ ] Generate test code coverage report and publish as an artifact
- [ ] All the lints pass (only informative, doesn't block pull requests)
- [ ] Build documentation and publish as an artifact
- [ ] Store benchmark results on a separate branch
- [ ] Automatic deployment to server
- [ ] Publish to crates.io on each release

#### Containerization

- [ ] Create a Docker image for development
- [ ] Create a Docker image for production deployment

### Core Library

- [x] Hybrid directed graph + 2D matrix representation
- [x] Graph builder for fluent programmatic construction
- [x] Mapping between graphs and lists of operations
- [ ] Accept common gate aliases to build operation lists
- [X] Validate operations on construction

#### Structural Validation

- [x] Validate dangling unconnected nodes
- [x] Validate that mirrored edges are correct
- [x] Validate duplicated edges

#### Semantic Validation

- [x] Create a schema for each gate type and its graph representation
- [x] Validate the graph's semantic consistency by using the gate schemas
- [ ] Validate that all angles in the graph are finite values
- [ ] Validate symmetrical gates hold the same data in each node

#### Graph Display

- [x] Display graph as a list of nodes and edges
- [x] Display graph as a 2D grid
- [x] Display a graph's unitary matrix representation
- [ ] Optional colorized display

#### Circuit Simplification

- [ ] Data model for pattern-based circuit simplification
- [ ] Circuit simplification algorithm
- [ ] Per-qubit gate cache to skip rules with gates not found in the pattern
- [ ] Skip rules bigger than the circuit's qubit count
- [ ] Use gate commutation to find more patterns

#### Supported Gate Operations

##### Single-qubit gates (17)

- [x] ID (Identity)
- [x] H (Hadamard)
- [x] X, Y, Z (Pauli)
- [x] P (Generic phase)
- [x] RX, RY, RZ (Rotation)
- [x] S, Sdg, T, Tdg (Phase shift)
- [x] SX, SY (Square root)
- [x] Measure (Measurement)

##### Two-qubit Gates (6)

- [x] Swap
- [x] CH (Controlled Hadamard)
- [x] CX, CY, CZ (Controlled Pauli)
- [x] CP (Controlled phase)
- [ ] CRX, CRY, CRZ (Controlled rotation)

##### Three-qubit Gates (4)

- [x] CSwap (Fredkin)
- [x] CCX (Toffoli)
- [ ] CCY, CCZ

##### Any Size (1)

- [ ] Generic unitary gates (kept as is)

### Format Conversion (converter)

- [x] JSON parsing and serialization
- [X] Customizable JSON pretty-printing
- [ ] MessagePack parsing and serialization
- [ ] Binary format
- [ ] Base64 encoding

### Circuit Visualization (presenter)

- [x] Color-coded gates and edges by type
- [x] Graphviz GV output
- [x] Graphviz PNG output
- [x] Graphviz SVG output

### Code Generation (codegen)

- [ ] Generate Qiskit code
- [ ] Generate OpenQASM 2 code

### Circuit Analysis (analyzer)

- [ ] Define set of metrics to analyze
- [ ] Analysis implementation

### Execution Estimation (estimator)

- [ ] Execution time estimation
- [ ] Cost estimation
- [ ] Implement cache for web scraping

### Storage (storage)

- [ ] Rule storage for simplification patterns
- [ ] JSON-based rule database
- [ ] Load and cache rules at startup

### Client Facade and Service Ports (facade and ports)

- [x] Request and response DTOs
- [x] Use case orchestration using ports
- [x] Display use case
- [x] Simplify use case
- [x] Present use case
- [x] Parse use case
- [x] Serialize use case
- [ ] Generate code use case
- [ ] Analyze use case
- [ ] Estimate use case

### Python Bindings (qiskit)

- [X] Documented and typed .pyi stubs
- [X] Conversion to and from Qiskit's QuantumCircuit

#### Expose bindings

- [X] Graph display (library)
- [X] Circuit simplification (library)
- [X] Format conversion (converter)
- [X] Circuit visualization (presenter)
- [ ] Code generation (codegen)
- [ ] Circuit analysis (analyzer)
- [ ] Execution estimation (estimator)

### CLI (cli)

- [ ] Load circuit from file (using converter)
- [ ] Save circuit to file (using converter)
- [ ] Display circuit as text/grid/matrix
- [ ] Simplify circuit
- [ ] Generate graph visualization
- [ ] Generate code (using codegen)
- [ ] Analyze circuit
- [ ] Estimate execution statistics
- [ ] Support for batch processing of multiple circuits

### REST API Server (server)

- [ ] GET /health - Health check
- [ ] POST /display - Display circuit
- [ ] POST /simplify - Simplify circuit
- [ ] POST /present - Generate visualization
- [ ] POST /parse - Parse circuit from format
- [ ] POST /serialize - Serialize circuit to format
- [ ] POST /generate - Generate code
- [ ] POST /analyze - Analyze circuit
- [ ] POST /estimate - Estimate execution statistics
- [ ] Allow passing different formats for circuits

## Non-Functional Requirements

### Performance

- [ ] Design scalable load and stress tests
- [ ] Implement performance tests and result storage

### Reliability

- [ ] Over 80% test coverage in the modules with the most logic
- [x] Error handling with thiserror
- [ ] Validate every struct at build time (constructor or builder)
- [ ] Property-based tests for core data structures
- [ ] Fuzz testing for parsers
- [ ] Integration test simulating all the use cases

### Usability

- [ ] Python bindings that feel Python-native
- [ ] CLI interface with useful options for automatized tasks
- [ ] REST API designed to be easily consumed by other tools or frontends

### Maintainability

- [x] Code follows Rust conventions
- [x] Strict clippy linting and formatting
- [ ] Strict commit message checking
- [x] Modular crate architecture
- [x] Clear separation of concerns (ports/adapters)
- [ ] Use Sonarqube as a code quality tool
- [ ] Document every non-private API in the code
- [ ] Document every module

### Security

- [x] No secrets in code
- [ ] Use of HTTPS for the REST API
- [ ] Use environment variables for configurations
- [x] Keep workspace dependencies up-to-date
