use getset::{CopyGetters, Getters};
use inew::New;

use crate::GateType;

/// An anchor point used for pattern matching.
///
/// It's the first node that is matched, and it helps reduce the number of permutations that need to be checked.
#[derive(Debug, Clone, Copy, Getters, CopyGetters, New)]
#[new(pub, const)]
pub struct Anchor {
    #[get_copy = "pub"]
    row: usize,
    #[get_copy = "pub"]
    column: usize,
    #[get_copy = "pub"]
    gate_type: GateType,
}
