use std::{array, fmt};

use getset::CopyGetters;
use inew::New;

/// Represents a (row, column) position in a `Graph`.
///
/// Its values are guaranteed to be non-negative.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, CopyGetters, New)]
#[get_copy = "pub"]
#[new(pub, const)]
#[must_use]
pub struct Position {
    /// The row, equivalent to the qubit index.
    row: usize,
    /// The column index.
    column: usize,
}

impl IntoIterator for Position {
    type Item = usize;
    type IntoIter = array::IntoIter<usize, 2>;

    /// Iterate over the position's coordinates, first the row and then the column.
    fn into_iter(self) -> Self::IntoIter {
        [self.row, self.column].into_iter()
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.row, self.column)
    }
}
