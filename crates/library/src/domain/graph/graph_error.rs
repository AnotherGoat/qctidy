use thiserror::Error;

use crate::{EdgeType, GateType, Position};

/// Error that can occur while working with a `Graph`.
#[derive(Debug, Error)]
pub enum GraphError {
    #[error("Node already exists at {position}")]
    NodeAlreadyExists { position: Position },
    #[error("Node at {position} does not exist")]
    NodeNotFound { position: Position },
    #[error("Start and end positions cannot be equal")]
    NullMove,
    #[error("An edge starts from a non-existent node at {from}")]
    DanglingStartNode { from: Position },
    #[error("An edge of type {edge_type} end points to a non-existent node at {from} to {to}")]
    DanglingEndNode {
        edge_type: EdgeType,
        from: Position,
        to: Position,
    },
    #[error(
        "An edge of type {edge_type} from {from} to {to} is missing its symmetrical counterpart"
    )]
    MissingReverseEdge {
        edge_type: EdgeType,
        from: Position,
        to: Position,
    },
    #[error("An edge of type {edge_type} from {from} to {to} exists more than once")]
    DuplicateEdge {
        edge_type: EdgeType,
        from: Position,
        to: Position,
    },
    #[error("Invalid edge {edge_type:?} from {from} to {to} for gate {gate:?}")]
    InvalidEdgeForGate {
        gate: GateType,
        edge_type: EdgeType,
        from: Position,
        to: Position,
    },
    #[error("Invalid angle for gate {gate:?} at {position}")]
    InvalidAngle { gate: GateType, position: Position },
    #[error("Invalid bit for gate {gate:?} at {position}")]
    InvalidBit { gate: GateType, position: Position },
    #[error("Invalid gate structure")]
    InvalidGateStructure,
}
