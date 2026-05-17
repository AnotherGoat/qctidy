use bit_vec::BitVec;

use crate::Position;

/// A map which tracks which positions in a graph are occupied.
///
/// Useful for pattern matching algorithms.
#[derive(Debug, Clone)]
pub(super) struct OccupancyMap {
    occupied: Vec<BitVec>,
}

impl OccupancyMap {
    /// Create a new `OccupancyMap` with the specified dimensions.
    pub(super) fn new(rows: usize, columns: usize) -> Self {
        Self {
            occupied: vec![BitVec::from_elem(columns, false); rows],
        }
    }

    /// Check whether a position is occupied or not.
    pub(super) fn is_occupied(&self, position: Position) -> bool {
        self.occupied[position.row()][position.column()]
    }

    /// Check whether a set of positions is occupied or not.
    pub(super) fn can_occupy(&self, positions: &[Position]) -> bool {
        positions
            .iter()
            .all(|&position| !self.is_occupied(position))
    }

    /// Occupy a position.
    pub(super) fn occupy(&mut self, position: Position) {
        self.occupied[position.row()].set(position.column(), true);
    }

    /// Occupy a set of positions.
    pub(super) fn occupy_all(&mut self, positions: &[Position]) {
        for position in positions {
            self.occupy(*position);
        }
    }
}
