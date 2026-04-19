use std::fmt;

/// The types of edges between graph nodes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EdgeType {
    /// Edge to the node on the left (paired with Right).
    Left,
    /// Edge to the node on the right (paired with Left).
    Right,
    /// Edge from controller to target (paired with ControlledBy).
    ///
    /// Used by all controlled gates.
    Targets,
    /// Edge from target to controller (paired with Targets).
    ///
    /// Used by all controlled gates.
    ControlledBy,
    /// Connects nodes that are swapped with each other (bidirectional).
    ///
    /// Only used in Swap and CSwap gates.
    SwapsWith,
    /// Connects nodes that work together (bidirectional).
    ///
    /// Used in CZ and CCZ gates to make them work symmetrically.
    /// Also used for storing relationships between control or target qubits that are part of the same gate.
    /// For example, a CCX gate uses WorksWith to associate its two control qubits.
    WorksWith,
}

impl fmt::Display for EdgeType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        use EdgeType::*;

        let name = match self {
            Left => "left",
            Right => "right",
            Targets => "targets",
            ControlledBy => "controlled_by",
            SwapsWith => "swaps_with",
            WorksWith => "works_with",
        };

        write!(formatter, "{}", name)
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

    /// Check whether this edge type is bidirectional or not.
    pub(crate) fn is_bidirectional(self) -> bool {
        matches!(self.opposite(), None)
    }

    /// Check whether this edge is related to the node's position or not.
    ///
    /// Positional edges never define a gate's structure.
    pub(crate) fn is_positional(self) -> bool {
        use EdgeType::*;
        matches!(self, Left | Right)
    }

    /// Check whether this edge is related to a gate's semantic structure or not.
    pub(crate) fn is_semantic(self) -> bool {
        !self.is_positional()
    }
}
