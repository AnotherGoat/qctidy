use std::fmt;

use getset::CopyGetters;

use crate::{
    domain::{GateType, Position},
    utils::format,
};

/// A read-only view of a node in a QuantumGraph.
#[derive(Debug, Clone, PartialEq, CopyGetters)]
#[get_copy = "pub"]
pub struct GraphNodeView {
    /// The type of the quantum gate in this node.
    r#type: GateType,
    /// The position of this node in the graph.
    position: Position,
    /// Rotation or phase angle (only for rotation and phase gates).
    angle: Option<f64>,
    /// Classical bit where the measured result is stored (only for measurement gates).
    bit: Option<usize>,
}

impl fmt::Display for GraphNodeView {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let angle_data = self
            .angle
            .map(|angle| format!(" (angle={})", format::format_angle(angle)))
            .unwrap_or_default();

        let bit_data = self
            .bit
            .map(|bit| format!(" (bit={})", bit))
            .unwrap_or_default();

        write!(
            formatter,
            "{} at {}{}{}",
            self.r#type, self.position, angle_data, bit_data
        )
    }
}

impl GraphNodeView {
    /// Create a new GraphNodeView.
    pub fn new(
        r#type: GateType,
        position: Position,
        angle: Option<f64>,
        bit: Option<usize>,
    ) -> Self {
        Self {
            r#type,
            position,
            angle,
            bit,
        }
    }
}
