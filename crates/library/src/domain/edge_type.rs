use std::fmt;

use strum_macros::EnumString;

/// The types of edges between graph nodes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumString)]
#[strum(serialize_all = "snake_case", ascii_case_insensitive)]
pub enum EdgeType {
    /// Edge to the node on the left (paired with Right).
    Left,
    /// Edge to the node on the right (paired with Left).
    Right,
    /// Connects nodes that are swapped with each other (bidirectional).
    ///
    /// Only used in Swap and CSwap gates.
    SwapsWith,
    /// Edge from controller to target (paired with ControlledBy).
    ///
    /// Used by all controlled gates.
    Targets,
    /// Edge from target to controller (paired with Targets).
    ///
    /// Used by all controlled gates.
    ControlledBy,
    /// Connects nodes that work together (bidirectional).
    ///
    /// Used in CZ and CCZ gates to make them work symmetrically.
    /// Also used for storing relationships between control or target qubits that are part of the same gate.
    /// For example, a CCX gate uses WorksWith to associate its two control qubits.
    WorksWith,
}

impl fmt::Display for EdgeType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.to_string())
    }
}

impl EdgeType {
    /// Get the opposite edge type, or None if the edge is bidirectional.
    pub(crate) fn opposite(self) -> Option<Self> {
        use EdgeType::*;

        match self {
            Left => Some(Right),
            Right => Some(Left),
            Targets => Some(ControlledBy),
            ControlledBy => Some(Targets),
            _ => None,
        }
    }

    /// Check whether this edge is related to the node's position or not.
    pub(crate) fn is_positional(self) -> bool {
        use EdgeType::*;
        matches!(self, Left | Right)
    }

    /// Check whether this edge type is bidirectional or not.
    pub(crate) fn is_bidirectional(self) -> bool {
        use EdgeType::*;
        matches!(self, SwapsWith | WorksWith)
    }
}
