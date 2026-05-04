pub(crate) mod application;
/// Contains domain structures used to represent a `Graph`.
pub(crate) mod domain;
pub(crate) mod interface;

pub use application::graph_builder::GraphBuilder;
pub use domain::graph::{
    Graph, edge_type::EdgeType, gate_type::GateType, graph_error::GraphError, position::Position,
};
pub use domain::math::{ABSOLUTE_TOLERANCE, EPSILON, RELATIVE_TOLERANCE};
pub use interface::display::{
    angle_formatter::{AngleFormat, PiFormat},
    graph_display::DiracFormat,
};
pub use interface::projection::{
    contextual_node_view::ContextualNodeView, edge_view::EdgeView, node_view::NodeView,
};

pub mod dto {
    pub use crate::interface::dto::gate_operation::{GateOperation, GateOperationError};

    pub mod mapper {
        pub use crate::interface::dto::mapper::{graph_to_operations, operations_to_graph};
    }
}

pub mod formatter {
    pub use crate::interface::display::angle_formatter::format as format_angle;
}

pub mod simplifier {
    pub use crate::application::simplifier::simplify;
}

pub mod math {
    pub use crate::domain::math::{ABSOLUTE_TOLERANCE, RELATIVE_TOLERANCE, are_floats_equal};
}
