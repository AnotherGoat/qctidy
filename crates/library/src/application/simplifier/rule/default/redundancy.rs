use std::sync::Arc;

use crate::{GraphBuilder, PatternRule, RuleGroup, RuleMetadata, RuleRegistry};

pub(crate) fn register(registry: &mut RuleRegistry) {
    registry.register(Arc::new(double_hadamard()));
}

pub(crate) fn double_hadamard() -> PatternRule {
    let lhs = GraphBuilder::default().push_h(0).push_h(0).build();
    let rhs = GraphBuilder::new(1).build();

    PatternRule::new(
        RuleMetadata::new(
            "double_hadamard",
            "Removes consecutive Hadamard gates.",
            RuleGroup::Redundancy,
            100,
        ),
        lhs,
        rhs,
    )
    .expect("Built-in rule should be valid")
}

pub(crate) fn double_x() -> PatternRule {
    let lhs = GraphBuilder::default().push_x(0).push_x(0).build();
    let rhs = GraphBuilder::new(1).build();

    PatternRule::new(
        RuleMetadata::new(
            "double_x",
            "Removes consecutive Hadamard gates.",
            RuleGroup::Redundancy,
            100,
        ),
        lhs,
        rhs,
    )
    .expect("Built-in rule should be valid")
}
