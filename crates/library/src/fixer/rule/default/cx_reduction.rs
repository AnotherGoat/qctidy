use std::sync::Arc;

use function_name::named;

use crate::{GraphBuilder, PatternRule, RuleCategory, RuleMetadata, RuleRegistry};

pub(crate) fn register(registry: &mut RuleRegistry) {
    registry.register_all(vec![
        Arc::new(cx_double_ladder_down()),
        Arc::new(cx_double_ladder_up()),
        Arc::new(cx_commutation_forward()),
        Arc::new(cx_commutation_backward()),
        Arc::new(cx_commutation_alternate_forward()),
        Arc::new(cx_commutation_alternate_backward()),
    ]);
}

#[named]
pub(crate) fn cx_double_ladder_down() -> PatternRule {
    let lhs = GraphBuilder::default()
        .push_cx(0, 1)
        .expect("All qubits should be different")
        .push_cx(1, 2)
        .expect("All qubits should be different")
        .push_cx(0, 1)
        .expect("All qubits should be different")
        .push_cx(1, 2)
        .expect("All qubits should be different")
        .build();

    let rhs = GraphBuilder::default()
        .push_cx(0, 2)
        .expect("All qubits should be different")
        .build();

    PatternRule::new(
        RuleMetadata::new(
            "X001",
            function_name!(),
            "Alternating CX ladder is equivalent to a single CX",
            RuleCategory::CxReduction,
            100,
        ),
        lhs,
        rhs,
    )
    .expect("Built-in rule should be valid")
}

#[named]
pub(crate) fn cx_double_ladder_up() -> PatternRule {
    let lhs = GraphBuilder::default()
        .push_cx(1, 2)
        .expect("All qubits should be different")
        .push_cx(0, 1)
        .expect("All qubits should be different")
        .push_cx(1, 2)
        .expect("All qubits should be different")
        .push_cx(0, 1)
        .expect("All qubits should be different")
        .build();

    let rhs = GraphBuilder::default()
        .push_cx(0, 2)
        .expect("All qubits should be different")
        .build();

    PatternRule::new(
        RuleMetadata::new(
            "X002",
            function_name!(),
            "Reversed alternating CX ladder is equivalent to a single CX",
            RuleCategory::CxReduction,
            100,
        ),
        lhs,
        rhs,
    )
    .expect("Built-in rule should be valid")
}

#[named]
pub(crate) fn cx_commutation_forward() -> PatternRule {
    let lhs = GraphBuilder::default()
        .push_cx(1, 2)
        .expect("All qubits should be different")
        .push_cx(0, 1)
        .expect("All qubits should be different")
        .push_cx(0, 2)
        .expect("All qubits should be different")
        .build();

    let rhs = GraphBuilder::default()
        .push_cx(0, 1)
        .expect("All qubits should be different")
        .push_cx(1, 2)
        .expect("All qubits should be different")
        .build();

    PatternRule::new(
        RuleMetadata::new(
            "X003",
            function_name!(),
            "Triangular CX interaction is equivalent to a linear CX chain",
            RuleCategory::CxReduction,
            100,
        ),
        lhs,
        rhs,
    )
    .expect("Built-in rule should be valid")
}

#[named]
pub(crate) fn cx_commutation_backward() -> PatternRule {
    let lhs = GraphBuilder::default()
        .push_cx(0, 2)
        .expect("All qubits should be different")
        .push_cx(1, 2)
        .expect("All qubits should be different")
        .push_cx(0, 1)
        .expect("All qubits should be different")
        .build();

    let rhs = GraphBuilder::default()
        .push_cx(0, 1)
        .expect("All qubits should be different")
        .push_cx(1, 2)
        .expect("All qubits should be different")
        .build();

    PatternRule::new(
        RuleMetadata::new(
            "X004",
            function_name!(),
            "Reordered triangular CX interaction is equivalent to a linear CX chain",
            RuleCategory::CxReduction,
            100,
        ),
        lhs,
        rhs,
    )
    .expect("Built-in rule should be valid")
}

#[named]
pub(crate) fn cx_commutation_alternate_forward() -> PatternRule {
    let lhs = GraphBuilder::default()
        .push_cx(0, 1)
        .expect("All qubits should be different")
        .push_cx(1, 2)
        .expect("All qubits should be different")
        .push_cx(0, 2)
        .expect("All qubits should be different")
        .build();

    let rhs = GraphBuilder::default()
        .push_cx(1, 2)
        .expect("All qubits should be different")
        .push_cx(0, 1)
        .expect("All qubits should be different")
        .build();

    PatternRule::new(
        RuleMetadata::new(
            "X005",
            function_name!(),
            "Triangular CX interaction can be reordered into a linear CX chain",
            RuleCategory::CxReduction,
            100,
        ),
        lhs,
        rhs,
    )
    .expect("Built-in rule should be valid")
}

#[named]
pub(crate) fn cx_commutation_alternate_backward() -> PatternRule {
    let lhs = GraphBuilder::default()
        .push_cx(0, 2)
        .expect("All qubits should be different")
        .push_cx(0, 1)
        .expect("All qubits should be different")
        .push_cx(1, 2)
        .expect("All qubits should be different")
        .build();

    let rhs = GraphBuilder::default()
        .push_cx(1, 2)
        .expect("All qubits should be different")
        .push_cx(0, 1)
        .expect("All qubits should be different")
        .build();

    PatternRule::new(
        RuleMetadata::new(
            "X006",
            function_name!(),
            "Reversed triangular CX interaction can be reordered into a linear CX chain",
            RuleCategory::CxReduction,
            100,
        ),
        lhs,
        rhs,
    )
    .expect("Built-in rule should be valid")
}
