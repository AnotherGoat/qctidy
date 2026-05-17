pub(crate) mod display;
/// Contains domain structures used to represent a `Graph`.
pub(crate) mod domain;
pub mod simplifier;

pub use display::{
    angle_formatter::{AngleFormat, PiFormat},
    graph_display::DiracFormat,
};
pub use domain::circuit::{Circuit, GateOperation, GateOperationError};
pub use domain::graph::graph_builder::{GraphBuilder, GraphBuilderError};
pub use domain::graph::{
    Graph, edge_type::EdgeType, gate_type::GateType, graph_error::GraphError, position::Position,
};
pub use domain::math::{ABSOLUTE_TOLERANCE, EPSILON, RELATIVE_TOLERANCE};
pub use domain::projection::{ContextualNodeView, EdgeView, NodeView};
pub use simplifier::pattern::pattern_match::PatternMatch;
pub use simplifier::rule::{
    RuleBuildError, SimplificationRule,
    configuration::{RuleConfiguration, RuleLevel},
    metadata::{RuleGroup, RuleId, RuleMetadata},
    pattern_rule::{PatternRule, PatternRuleSide},
    registry::RuleRegistry,
};

pub mod formatter {
    pub use crate::display::angle_formatter::format as format_angle;
}

pub mod math {
    pub use crate::domain::math::{ABSOLUTE_TOLERANCE, RELATIVE_TOLERANCE, are_floats_equal};
}
