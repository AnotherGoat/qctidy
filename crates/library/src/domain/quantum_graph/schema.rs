use crate::domain::{EdgeType, GateType};
use EdgeType::*;
use GateType::*;

/// A schema for a gate, which defines the number of graph nodes and edges required for it to be valid.
///
/// This is used to validate the internal semantic structure of `QuantumGraphs`.
/// A gate may be thought of as a molecule, where the nodes are atoms and the edges are bonds.
pub(super) struct GateSchema {
    /// The type of gate this schema is for.
    pub(super) r#type: GateType,
    /// The expected schema for node that is part of the gate.
    pub(super) nodes: &'static [NodeSchema],
    /// The expected edges between the nodes.
    pub(super) edges: &'static [EdgeSchema],
}

/// A schema for a graph node, part of a gate structure.
///
/// A node may be thought of as an atom.
pub(super) struct NodeSchema {
    /// Whether the node must have an angle or not.
    pub(super) has_angle: bool,
    /// Whether the node must have a bit or not.
    pub(super) has_bit: bool,
}

impl NodeSchema {
    pub(super) const fn new(has_angle: bool, has_bit: bool) -> NodeSchema {
        NodeSchema { has_angle, has_bit }
    }
}

/// A schema for an edge between graph nodes.
///
/// An edge may be thought of as a bond between two atoms.
pub(super) struct EdgeSchema {
    pub(super) edge_type: EdgeType,
    pub(super) from: usize,
    pub(super) to: usize,
}

impl EdgeSchema {
    pub(super) const fn new(edge_type: EdgeType, from: usize, to: usize) -> EdgeSchema {
        EdgeSchema {
            edge_type,
            from,
            to,
        }
    }
}

macro_rules! single_schema {
    ($gate_type:path) => {
        GateSchema {
            r#type: $gate_type,
            nodes: &[NodeSchema::new(false, false)],
            edges: &[],
        }
    };
}

macro_rules! rotation_schema {
    ($gate_type:path) => {
        GateSchema {
            r#type: $gate_type,
            nodes: &[NodeSchema::new(true, false)],
            edges: &[],
        }
    };
}

macro_rules! control_schema {
    ($gate_type:path) => {
        GateSchema {
            r#type: $gate_type,
            nodes: &[NodeSchema::new(false, false), NodeSchema::new(false, false)],
            edges: &[
                EdgeSchema::new(Targets, 0, 1),
                EdgeSchema::new(ControlledBy, 1, 0),
            ],
        }
    };
}

const ID_SCHEMA: GateSchema = single_schema!(ID);

const H_SCHEMA: GateSchema = single_schema!(H);

const X_SCHEMA: GateSchema = single_schema!(X);

const Y_SCHEMA: GateSchema = single_schema!(Y);

const Z_SCHEMA: GateSchema = single_schema!(Z);

const P_SCHEMA: GateSchema = rotation_schema!(P);

const RX_SCHEMA: GateSchema = rotation_schema!(RX);

const RY_SCHEMA: GateSchema = rotation_schema!(RY);

const RZ_SCHEMA: GateSchema = rotation_schema!(RZ);

const S_SCHEMA: GateSchema = single_schema!(S);

const SDG_SCHEMA: GateSchema = single_schema!(SDG);

const SX_SCHEMA: GateSchema = single_schema!(SX);

const SY_SCHEMA: GateSchema = single_schema!(SY);

const T_SCHEMA: GateSchema = single_schema!(T);

const TDG_SCHEMA: GateSchema = single_schema!(TDG);

const MEASURE_SCHEMA: GateSchema = GateSchema {
    r#type: Measure,
    nodes: &[NodeSchema::new(false, true)],
    edges: &[],
};

const SWAP_SCHEMA: GateSchema = GateSchema {
    r#type: Swap,
    nodes: &[NodeSchema::new(false, false), NodeSchema::new(false, false)],
    edges: &[
        EdgeSchema::new(SwapsWith, 0, 1),
        EdgeSchema::new(SwapsWith, 1, 0),
    ],
};

const CH_SCHEMA: GateSchema = control_schema!(CH);

const CX_SCHEMA: GateSchema = control_schema!(CX);

const CY_SCHEMA: GateSchema = control_schema!(CY);

const CZ_SCHEMA: GateSchema = GateSchema {
    r#type: CZ,
    nodes: &[NodeSchema::new(false, false), NodeSchema::new(false, false)],
    edges: &[
        EdgeSchema::new(WorksWith, 0, 1),
        EdgeSchema::new(WorksWith, 1, 0),
    ],
};

const CP_SCHEMA: GateSchema = GateSchema {
    r#type: CP,
    nodes: &[NodeSchema::new(false, false), NodeSchema::new(true, false)],
    edges: &[
        EdgeSchema::new(Targets, 0, 1),
        EdgeSchema::new(ControlledBy, 1, 0),
    ],
};

const CSWAP_SCHEMA: GateSchema = GateSchema {
    r#type: CSwap,
    nodes: &[
        NodeSchema::new(false, false),
        NodeSchema::new(false, false),
        NodeSchema::new(false, false),
    ],
    edges: &[
        EdgeSchema::new(Targets, 0, 1),
        EdgeSchema::new(ControlledBy, 1, 0),
        EdgeSchema::new(Targets, 0, 2),
        EdgeSchema::new(ControlledBy, 2, 0),
        EdgeSchema::new(SwapsWith, 1, 2),
        EdgeSchema::new(SwapsWith, 2, 1),
    ],
};

const CCX_SCHEMA: GateSchema = GateSchema {
    r#type: CCX,
    nodes: &[
        NodeSchema::new(false, false),
        NodeSchema::new(false, false),
        NodeSchema::new(false, false),
    ],
    edges: &[
        EdgeSchema::new(Targets, 0, 2),
        EdgeSchema::new(ControlledBy, 2, 0),
        EdgeSchema::new(Targets, 1, 2),
        EdgeSchema::new(ControlledBy, 2, 1),
        EdgeSchema::new(WorksWith, 0, 1),
        EdgeSchema::new(WorksWith, 1, 0),
    ],
};

const CCZ_SCHEMA: GateSchema = GateSchema {
    r#type: CCZ,
    nodes: &[
        NodeSchema::new(false, false),
        NodeSchema::new(false, false),
        NodeSchema::new(false, false),
    ],
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
    pub(super) const fn schema(self) -> &'static GateSchema {
        match self {
            ID => &ID_SCHEMA,
            H => &H_SCHEMA,
            X => &X_SCHEMA,
            Y => &Y_SCHEMA,
            Z => &Z_SCHEMA,
            P => &P_SCHEMA,
            RX => &RX_SCHEMA,
            RY => &RY_SCHEMA,
            RZ => &RZ_SCHEMA,
            S => &S_SCHEMA,
            SDG => &SDG_SCHEMA,
            SX => &SX_SCHEMA,
            SY => &SY_SCHEMA,
            T => &T_SCHEMA,
            TDG => &TDG_SCHEMA,
            Measure => &MEASURE_SCHEMA,
            Swap => &SWAP_SCHEMA,
            CH => &CH_SCHEMA,
            CX => &CX_SCHEMA,
            CY => &CY_SCHEMA,
            CZ => &CZ_SCHEMA,
            CP => &CP_SCHEMA,
            CSwap => &CSWAP_SCHEMA,
            CCX => &CCX_SCHEMA,
            CCZ => &CCZ_SCHEMA,
        }
    }
}
