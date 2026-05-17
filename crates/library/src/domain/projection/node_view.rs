use getset::CopyGetters;
use inew::New;

use crate::{GateType, Position};

#[derive(Debug, Clone, Copy, PartialEq, CopyGetters, New)]
#[get_copy = "pub"]
#[new(pub, const)]
#[must_use]
pub struct NodeView {
    r#type: GateType,
    position: Position,
    angle: Option<f64>,
    bit: Option<usize>,
}
