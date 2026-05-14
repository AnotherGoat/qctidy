# Changes from Python to Rust

The project now has an actual architecture, based on Hexagonal Architecture. It implements ports and adapters for every optional feature, making it very modular and easy to add new features without breaking the rest of the code.

The `Graph` is now hand-written in pure Rust, instead of using a library implementation such as [NetworkX](https://networkx.org/en/)'s `DiGraph`. This allows more control over the stored data and is more reliable than using Python's `dict` for every node.

Internally, the `Graph` stores:

- Every node as a `HashMap<Position, NodeData>`.
- Outgoing edges as a `HashMap<Position, Vec<EdgeData>>`.
- Incoming edges as a `HashMap<Position, Vec<EdgeData>>`.

Outgoing edges are the single source of truth for edge connections, and incoming edges are only used for fast lookups of the other end of an edge.

Nodes are now stored in a sparse way, which means that empty spaces don't hold an `ID` gate anymore. This makes pattern matching and simplification algorithms much easier to implement.

Redundant edge types have been removed:

- `Left` is the source of a `Right` edge.
- `ControlledBy` is the source of a `Targets` edge.
- `SwapsWith` and `WorksWith` remain bidirectional.

Edge types are now properly classified:

- `Right` is structural, since it represents a time step.
- `Targets`, `SwapsWith` and `WorksWith` are semantic, since it represents relationships between nodes from the same operation.

Validations are much stricter, and `GateSchema` has been created to validate semantic relationships.

Some gates have differences in their graph representation:

- Since `CP` is a bidirectional gate, it now stores the angle on both sides, and its edges have been replaced by `WorksWith`.

Operations are now stored in the `GateOperation` enum, which is essentially a tagged union and more strongly typed than [Pydantic](https://pydantic.dev/docs/)'s `BaseModel`.
