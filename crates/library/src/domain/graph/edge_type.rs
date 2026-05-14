use std::fmt;

/// The types of edges between graph nodes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EdgeType {
    /// Edge to the node on the right, representing a time step.
    Right,
    /// Edge from controller to target.
    ///
    /// Used by all controlled gates.
    Targets,
    /// Connects nodes that are swapped with each other (bidirectional).
    ///
    /// Only used in Swap and `CSwap` gates.
    SwapsWith,
    /// Connects nodes that work together (bidirectional).
    ///
    /// Used in CZ, CP and CCZ gates to make them work symmetrically, because they only affect |11> or |111> states.
    /// Also used for storing relationships between control or target qubits that are part of the same gate.
    /// For example, a CCX gate uses `WorksWith` to associate its two control qubits.
    WorksWith,
}

impl fmt::Display for EdgeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use EdgeType::*;

        let name = match *self {
            Right => "right",
            Targets => "targets",
            SwapsWith => "swaps_with",
            WorksWith => "works_with",
        };

        write!(f, "{name}")
    }
}

impl EdgeType {
    /// Check whether this edge type is bidirectional or not.
    pub(crate) const fn is_bidirectional(self) -> bool {
        use EdgeType::*;
        matches!(self, SwapsWith | WorksWith)
    }

    /// Check whether this edge is related to the node's position or not.
    ///
    /// Positional edges never define a gate operation's structure, but they are required to join time steps in the graph.
    pub(crate) const fn is_structural(self) -> bool {
        matches!(self, Self::Right)
    }

    /// Check whether this edge is related to a gate's semantic structure or not.
    pub(crate) const fn is_semantic(self) -> bool {
        !self.is_structural()
    }
}
