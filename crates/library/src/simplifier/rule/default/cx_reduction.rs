use std::sync::Arc;

use function_name::named;

use crate::{GraphBuilder, PatternRule, RuleGroup, RuleMetadata, RuleRegistry};

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
            function_name!(),
            "Collapses alternating CX ladder into a single CX.",
            RuleGroup::CxReduction,
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
            function_name!(),
            "Collapses reversed alternating CX ladder into a single CX.",
            RuleGroup::CxReduction,
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
            function_name!(),
            "Compacts triangular CX interaction into a linear CX chain.",
            RuleGroup::CxReduction,
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
            function_name!(),
            "Compacts reordered triangular CX interaction into a linear CX chain.",
            RuleGroup::CxReduction,
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
            function_name!(),
            "Reorders and compacts triangular CX interaction.",
            RuleGroup::CxReduction,
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
            function_name!(),
            "Reorders and compacts reversed triangular CX interaction.",
            RuleGroup::CxReduction,
            100,
        ),
        lhs,
        rhs,
    )
    .expect("Built-in rule should be valid")
}
