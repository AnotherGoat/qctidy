use crate::{EdgeType, GateType};
use EdgeType::*;
use GateType::*;
use getset::{CopyGetters, Getters};
use inew::New;

/// A schema for a gate, which defines the number of graph nodes and edges required for it to be valid.
///
/// This is used to validate the internal semantic structure of `Graphs`.
/// A gate may be thought of as a molecule, where the nodes are atoms and the edges are bonds.
#[derive(Getters, CopyGetters)]
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
pub(super) enum NodeSchema {
    /// The node includes no extra data.
    NoData,
    /// The node has an angle (for phase or rotation gates).
    Angle,
    /// The node has a bit (for measurement gates).
    Bit,
}

impl NodeSchema {
    pub(super) const fn has_angle(&self) -> bool {
        matches!(self, Self::Angle)
    }

    pub(super) const fn has_bit(&self) -> bool {
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

const SINGLE_SCHEMA: GateSchema = GateSchema {
    nodes: &[NodeSchema::NoData],
    edges: &[],
};

const ROTATION_SCHEMA: GateSchema = GateSchema {
    nodes: &[NodeSchema::Angle],
    edges: &[],
};

const MEASURE_SCHEMA: GateSchema = GateSchema {
    nodes: &[NodeSchema::Bit],
    edges: &[],
};

const SWAP_SCHEMA: GateSchema = GateSchema {
    nodes: &[NodeSchema::NoData, NodeSchema::NoData],
    edges: &[
        EdgeSchema::new(SwapsWith, 0, 1),
        EdgeSchema::new(SwapsWith, 1, 0),
    ],
};

const CONTROL_SCHEMA: GateSchema = GateSchema {
    nodes: &[NodeSchema::NoData, NodeSchema::NoData],
    edges: &[EdgeSchema::new(Targets, 0, 1)],
};

const CZ_SCHEMA: GateSchema = GateSchema {
    nodes: &[NodeSchema::NoData, NodeSchema::NoData],
    edges: &[
        EdgeSchema::new(WorksWith, 0, 1),
        EdgeSchema::new(WorksWith, 1, 0),
    ],
};

const CP_SCHEMA: GateSchema = GateSchema {
    nodes: &[NodeSchema::Angle, NodeSchema::Angle],
    edges: &[
        EdgeSchema::new(WorksWith, 0, 1),
        EdgeSchema::new(WorksWith, 1, 0),
    ],
};

const CSWAP_SCHEMA: GateSchema = GateSchema {
    nodes: &[NodeSchema::NoData, NodeSchema::NoData, NodeSchema::NoData],
    edges: &[
        EdgeSchema::new(Targets, 0, 1),
        EdgeSchema::new(Targets, 0, 2),
        EdgeSchema::new(SwapsWith, 1, 2),
        EdgeSchema::new(SwapsWith, 2, 1),
    ],
};

const CCX_SCHEMA: GateSchema = GateSchema {
    nodes: &[NodeSchema::NoData, NodeSchema::NoData, NodeSchema::NoData],
    edges: &[
        EdgeSchema::new(Targets, 0, 2),
        EdgeSchema::new(Targets, 1, 2),
        EdgeSchema::new(WorksWith, 0, 1),
        EdgeSchema::new(WorksWith, 1, 0),
    ],
};

const CCZ_SCHEMA: GateSchema = GateSchema {
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
    pub(super) const fn schema(self) -> &'static GateSchema {
        match self {
            ID | H | X | Y | Z | S | SDG | SX | SY | T | TDG => &SINGLE_SCHEMA,
            P | RX | RY | RZ => &ROTATION_SCHEMA,
            Measure => &MEASURE_SCHEMA,
            Swap => &SWAP_SCHEMA,
            CH | CX | CY => &CONTROL_SCHEMA,
            CZ => &CZ_SCHEMA,
            CP => &CP_SCHEMA,
            CSwap => &CSWAP_SCHEMA,
            CCX => &CCX_SCHEMA,
            CCZ => &CCZ_SCHEMA,
        }
    }
}
