# Architecture

QCTidy is organized based on hexagonal architecture (ports and adapters).
Dependencies point inward:

- Any client code and composition roots depend on the application layer.
- Application layer depends on ports and the domain.
- Adapters implement ports and use the domain.
- Domain depends on nothing else.

![High level architecture diagram](images/architecture_high.drawio.png)

The diagram above shows the target "ideal" architecture for the project.
It omits a few wiring edges to stay readable without cluttering the diagram with unnecessary lines.

## Domain layer

The core of the project and the lowest level of abstraction. It has no
dependencies on any other layer.

- **`library`** (`qctidy`): Core data structures, general utilities and rule
  storage, detection and fixing. Circuits are represented as a hybrid directed
  graph and 2D matrix: each `Position` is a `(qubit, time step)` pair, which can
  be seen as a `(row, column)` of the matrix.

## Port layer

Abstract interfaces and shared DTOs. It depends only on the domain layer.

- **`ports`** (`qctidy-ports`): Traits that allow multiple implementations and
  structs that define DTOs, including error types.

## Adapter layer

Each adapter implements exactly one port and uses the domain. None of them
knows about the others.

- **`analyzer`** (`AnalyzerPort`): Calculates metrics that could be used to
  estimate a quantum circuit's quality.
- **`codegen`** (`CodegenPort`): Generates **Qiskit** (Python) and
  **OpenQASM 3** code, either from a full circuit or a single operation.
- **`converter`** (`ConverterPort`): Parses and serializes circuits to and from
  data interchange formats such as JSON, XML, MessagePack, and CBOR, with
  format versioning for future-proofing.
- **`estimator`** (`EstimatorPort`): Estimates execution time and cost per
  provider (for example AWS and IBM) for a given number of shots. This is an
  experimental module, added only to show what can be done with the
  architecture.
- **`presenter`** (`PresenterPort`): Renders circuits as graphs (the IR used
  across the project) with Graphviz as GV, PNG, or SVG.
- **`python_ast`** (`PythonAstPort`): Parses **Qiskit** (Python) source with
  tree-sitter into scopes, circuits, gates, issues, and a per-operation source
  map.

## Application layer

The use-case layer, which depends on the domain and ports only. It orchestrates
the domain and the ports, and is generic over the ports, so it never depends on
a concrete adapter. Potentially, there could be any number of implementations
of which this layer could choose one.

- **`facade`** (`qctidy-facade`): Contains use cases for various common
  operations, such as displaying, parsing/serializing, analyzing, checking or
  fixing circuits.

## Composition root layer

Entry points. They parse input, instantiate the adapters they need, and call the
facade use cases.

- **`cli`**: Command-line tool for static analysis and batch processing. It
  includes various subcommands for various use cases and linter-like options
  such as `qctidy check --fix`.
- **`tui`**: Lightweight terminal UI (ratatui/crossterm) for browsing and
  inspecting circuits, mainly meant to be used as a visual circuit building UI
  that can be used directly in servers.
- **`server`**: REST API (Axum) with OpenAPI/Swagger docs. It has endpoints for
  various use cases.
- **`qiskit`**: PyO3 bindings that expose the facade use cases to Python as the
  `qctidy` package.

## Clients

Examples of client programs, some of which may not be fully implemented, but
exist to showcase the strengths of the architecture.

- **`vscode`**: TypeScript extension for Visual Studio Code. Shows a sidebar
  with the AST analysis (scopes, circuits, gates, issues) and supports
  jump-to-line navigation. It runs the `qctidy ast` subcommand, which is why it
  depends on `cli`.
- **`client`** (Web UI): React/Vite frontend with a drag-and-drop gate editor
  that talks to `server`, showcasing circuit fixing in a visual way.
- **`External code`**: Any user-created Python code that imports the `qiskit`
  bindings and may potentially use cloud services such as `IBM Cloud`.
- **`IBM Cloud`**: Real quantum hardware reachable from external code, which
  could be replaced by any other quantum cloud provider if bindings for its
  library are implemented.
