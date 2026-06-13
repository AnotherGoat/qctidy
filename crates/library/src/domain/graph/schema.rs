use crate::{EdgeType, GATE_METADATAS, GateType};
use EdgeType::*;
use getset::{CopyGetters, Getters};
use inew::New;

/// A schema for a gate, which defines the number of graph nodes and edges required for it to be valid.
///
/// This is used to validate the internal semantic structure of `Graphs`.
/// A gate may be thought of as a molecule, where the nodes are atoms and the edges are bonds.
#[derive(Debug, Clone, Getters, CopyGetters)]
#[must_use]
pub(super) struct GateSchema {
    #[get = "pub"]
    nodes: &'static [NodeSchema],
    #[get = "pub"]
    edges: &'static [EdgeSchema],
}

/// A schema for a graph node, part of a gate structure.
///
/// A node may be thought of as an atom.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[must_use]
pub(super) enum NodeSchema {
    /// The node includes no extra data.
    NoData,
    /// The node has an angle (for phase or rotation gates).
    Angle,
    /// The node has three angles (for the U gate: theta, phi, lambda).
    TripleAngle,
    /// The node has a bit (for measurement gates).
    Bit,
}

impl NodeSchema {
    pub(super) const fn has_angle(self) -> bool {
        matches!(self, Self::Angle | Self::TripleAngle)
    }

    pub(super) const fn has_bit(self) -> bool {
        matches!(self, Self::Bit)
    }
}

/// A schema for an edge between graph nodes.
///
/// An edge may be thought of as a bond between two atoms.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, CopyGetters, New)]
#[get_copy = "pub"]
#[new(pub(super), const)]
#[must_use]
pub(super) struct EdgeSchema {
    r#type: EdgeType,
    from_index: usize,
    to_index: usize,
}

pub(super) const SINGLE_SCHEMA: GateSchema = GateSchema {
    nodes: &[NodeSchema::NoData],
    edges: &[],
};

pub(super) const ROTATION_SCHEMA: GateSchema = GateSchema {
    nodes: &[NodeSchema::Angle],
    edges: &[],
};

pub(super) const MEASURE_SCHEMA: GateSchema = GateSchema {
    nodes: &[NodeSchema::Bit],
    edges: &[],
};

pub(super) const U_SCHEMA: GateSchema = GateSchema {
    nodes: &[NodeSchema::TripleAngle],
    edges: &[],
};

pub(super) const SWAP_SCHEMA: GateSchema = GateSchema {
    nodes: &[NodeSchema::NoData, NodeSchema::NoData],
    edges: &[
        EdgeSchema::new(SwapsWith, 0, 1),
        EdgeSchema::new(SwapsWith, 1, 0),
    ],
};

pub(super) const CONTROL_SCHEMA: GateSchema = GateSchema {
    nodes: &[NodeSchema::NoData, NodeSchema::NoData],
    edges: &[EdgeSchema::new(Targets, 0, 1)],
};

pub(super) const CZ_SCHEMA: GateSchema = GateSchema {
    nodes: &[NodeSchema::NoData, NodeSchema::NoData],
    edges: &[
        EdgeSchema::new(WorksWith, 0, 1),
        EdgeSchema::new(WorksWith, 1, 0),
    ],
};

pub(super) const CP_SCHEMA: GateSchema = GateSchema {
    nodes: &[NodeSchema::Angle, NodeSchema::Angle],
    edges: &[
        EdgeSchema::new(WorksWith, 0, 1),
        EdgeSchema::new(WorksWith, 1, 0),
    ],
};

pub(super) const CSWAP_SCHEMA: GateSchema = GateSchema {
    nodes: &[NodeSchema::NoData, NodeSchema::NoData, NodeSchema::NoData],
    edges: &[
        EdgeSchema::new(Targets, 0, 1),
        EdgeSchema::new(Targets, 0, 2),
        EdgeSchema::new(SwapsWith, 1, 2),
        EdgeSchema::new(SwapsWith, 2, 1),
    ],
};

pub(super) const CCX_SCHEMA: GateSchema = GateSchema {
    nodes: &[NodeSchema::NoData, NodeSchema::NoData, NodeSchema::NoData],
    edges: &[
        EdgeSchema::new(Targets, 0, 2),
        EdgeSchema::new(Targets, 1, 2),
        EdgeSchema::new(WorksWith, 0, 1),
        EdgeSchema::new(WorksWith, 1, 0),
    ],
};

pub(super) const CCZ_SCHEMA: GateSchema = GateSchema {
    nodes: &[NodeSchema::NoData, NodeSchema::NoData, NodeSchema::NoData],
    edges: &[
        EdgeSchema::new(WorksWith, 0, 1),
        EdgeSchema::new(WorksWith, 1, 0),
        EdgeSchema::new(WorksWith, 0, 2),
        EdgeSchema::new(WorksWith, 2, 0),
        EdgeSchema::new(WorksWith, 1, 2),
        EdgeSchema::new(WorksWith, 2, 1),
    ],
};

impl GateType {
    /// The node and edge structure of this gate.
    pub(super) fn schema(self) -> &'static GateSchema {
        GATE_METADATAS[usize::from(self)].schema()
    }
}
