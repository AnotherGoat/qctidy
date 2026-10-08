use getset::{CopyGetters, Getters};
use newgen::New;

use crate::{GateType, Position};

/// An anchor point used for pattern matching.
///
/// It's the first node that is matched, and it helps reduce the number of permutations that need to be checked.
#[derive(Debug, Clone, Copy, Getters, CopyGetters, New)]
#[new(pub, const)]
pub struct Anchor {
    #[get_copy = "pub"]
    position: Position,
    #[get_copy = "pub"]
    gate_type: GateType,
}
