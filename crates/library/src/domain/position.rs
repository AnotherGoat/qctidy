use std::fmt;

use getset::CopyGetters;

/// Represents a (row, column) position in a `Graph`.
///
/// Its values are guaranteed to be non-negative.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, CopyGetters)]
#[get_copy = "pub"]
pub struct Position {
    /// The row, equivalent to the qubit index.
    row: usize,
    /// The column index.
    column: usize,
}

impl Position {
    /// Create a new Position.
    pub const fn new(row: usize, column: usize) -> Self {
        Self { row, column }
    }
}

impl IntoIterator for Position {
    type Item = usize;
    type IntoIter = std::array::IntoIter<usize, 2>;

    /// Iterate over the position's coordinates, first the row and then the column.
    fn into_iter(self) -> Self::IntoIter {
        [self.row, self.column].into_iter()
    }
}

impl fmt::Display for Position {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "({}, {})", self.row, self.column)
    }
}
