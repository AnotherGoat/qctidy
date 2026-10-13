pub(crate) mod display;
/// Contains domain structures used to represent a `Graph`.
pub(crate) mod domain;
pub mod fixer;
pub mod random;

pub use display::{
    angle_formatter::{AngleFormat, PiFormat},
    graph_display::DiracFormat,
};
pub use domain::circuit::{Circuit, GateOperation, GateOperationError};
pub use domain::graph::gate_metadata::GATE_METADATAS;
pub use domain::graph::graph_builder::{GraphBuilder, GraphBuilderError};
pub use domain::graph::{
    Graph,
    edge_type::EdgeType,
    gate_metadata::GateMetadata,
    gate_type::{GateType, GateTypeError},
    graph_error::GraphError,
    position::Position,
};
pub use domain::math::{ABSOLUTE_TOLERANCE, EPSILON, RELATIVE_TOLERANCE};
pub use domain::projection::{ContextualNodeView, EdgeView, NodeView};
pub use fixer::Diagnostic;
pub use fixer::pattern::pattern_match::PatternMatch;
pub use fixer::rule::{
    CanonicalizationRule, Rule, RuleBuildError,
    configuration::{RuleConfiguration, RuleSeverity},
    metadata::{RuleCategory, RuleCode, RuleMetadata},
    pattern_rule::{PatternRule, PatternRuleSide},
    registry::RuleRegistry,
};

pub mod formatter {
    pub use crate::display::angle_formatter::format as format_angle;
}

pub mod math {
    pub use crate::domain::math::{ABSOLUTE_TOLERANCE, RELATIVE_TOLERANCE, are_floats_equal};
}

/// Lossy number conversions, exposed so other crates keep every unsafe cast in one place.
pub mod number {
    pub use crate::domain::number::{
        truncate_f64_to_i64, truncate_f64_to_usize, truncate_i64_to_f64, truncate_u64_to_f64,
        truncate_usize_to_f64, truncate_usize_to_isize, truncate_usize_to_u8,
        truncate_usize_to_u16,
    };
}
