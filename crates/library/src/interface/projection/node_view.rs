use getset::CopyGetters;
use inew::New;

use crate::{GateType, Position};

/// A read-only projection of a node in a `Graph`.
#[derive(Debug, Clone, Copy, PartialEq, CopyGetters, New)]
#[get_copy = "pub"]
#[new(pub, const)]
#[must_use]
pub struct NodeView {
    /// The type of the quantum gate in this node.
    r#type: GateType,
    /// The position of this node in the graph.
    position: Position,
    /// Rotation or phase angle (only for rotation and phase gates).
    angle: Option<f64>,
    /// Classical bit where the measured result is stored (only for measurement gates).
    bit: Option<usize>,
}
