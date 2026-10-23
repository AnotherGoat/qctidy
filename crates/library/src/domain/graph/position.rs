use std::fmt;

use getset::CopyGetters;
use newgen::New;

/// Represents a (qubit, time step) position in a `Graph`.
///
/// Its values are guaranteed to be non-negative.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, CopyGetters, New)]
#[get_copy = "pub"]
#[new(pub, const)]
#[must_use]
pub struct Position {
    /// The qubit index.
    qubit: usize,
    /// The time step index.
    time_step: usize,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.qubit, self.time_step)
    }
}
