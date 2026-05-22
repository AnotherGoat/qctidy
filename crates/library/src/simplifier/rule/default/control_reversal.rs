use std::sync::Arc;

use function_name::named;

use crate::{GraphBuilder, PatternRule, RuleGroup, RuleMetadata, RuleRegistry};

pub(crate) fn register(registry: &mut RuleRegistry) {
    registry.register_all(vec![Arc::new(cx_reversal())]);
}

#[named]
pub(crate) fn cx_reversal() -> PatternRule {
    let lhs = GraphBuilder::default()
        .push_h(0)
        .push_h(1)
        .push_cx(0, 1)
        .expect("All qubits should be different")
        .push_h(0)
        .push_h(1)
        .build();

    let rhs = GraphBuilder::default()
        .push_cx(1, 0)
        .expect("All qubits should be different")
        .build();

    PatternRule::new(
        RuleMetadata::new(
            function_name!(),
            "Reverses CX",
            RuleGroup::ControlReversal,
            100,
        ),
        lhs,
        rhs,
    )
    .expect("Built-in rule should be valid")
}
