use getset::CopyGetters;
use newgen::New;

use crate::{GateType, Position};

#[derive(Debug, Clone, Copy, PartialEq, CopyGetters, New)]
#[get_copy = "pub"]
#[new(pub, const)]
#[must_use]
pub struct NodeView {
    r#type: GateType,
    position: Position,
    theta: Option<f64>,
    phi: Option<f64>,
    lambda: Option<f64>,
    bit: Option<usize>,
}

impl NodeView {
    pub(crate) fn semantic_angle(&self) -> Option<f64> {
        match self.r#type() {
            GateType::RZ => self.phi(),
            _ => self.theta(),
        }
    }
}
