use std::sync::Arc;

use function_name::named;

use crate::{GraphBuilder, PatternRule, RuleGroup, RuleMetadata, RuleRegistry};

pub(crate) fn register(registry: &mut RuleRegistry) {
    registry.register_all(vec![
        Arc::new(hadamard_x_hadamard()),
        Arc::new(hadamard_z_hadamard()),
        Arc::new(hadamard_cx_hadamard()),
        Arc::new(hadamard_cz_hadamard()),
        Arc::new(hadamard_ccx_hadamard()),
    ]);
}

#[named]
pub(crate) fn hadamard_x_hadamard() -> PatternRule {
    let lhs = GraphBuilder::default()
        .push_h(0)
        .push_x(0)
        .push_h(0)
        .build();

    let rhs = GraphBuilder::new(1).push_z(0).build();

    PatternRule::new(
        RuleMetadata::new(
            function_name!(),
            "Transforms H X H into Z.",
            RuleGroup::BasisChange,
            90,
        ),
        lhs,
        rhs,
    )
    .expect("Built-in rule should be valid")
}

#[named]
pub(crate) fn hadamard_z_hadamard() -> PatternRule {
    let lhs = GraphBuilder::default()
        .push_h(0)
        .push_z(0)
        .push_h(0)
        .build();

    let rhs = GraphBuilder::new(1).push_x(0).build();

    PatternRule::new(
        RuleMetadata::new(
            function_name!(),
            "Transforms H Z H into X.",
            RuleGroup::BasisChange,
            90,
        ),
        lhs,
        rhs,
    )
    .expect("Built-in rule should be valid")
}

#[named]
pub(crate) fn hadamard_cx_hadamard() -> PatternRule {
    let lhs = GraphBuilder::default()
        .push_h(1)
        .push_cx(0, 1)
        .expect("All qubits should be different")
        .push_h(1)
        .build();

    let rhs = GraphBuilder::default()
        .push_cz(0, 1)
        .expect("All qubits should be different")
        .build();

    PatternRule::new(
        RuleMetadata::new(
            function_name!(),
            "Transforms H CX H into CZ.",
            RuleGroup::BasisChange,
            90,
        ),
        lhs,
        rhs,
    )
    .expect("Built-in rule should be valid")
}

#[named]
pub(crate) fn hadamard_cz_hadamard() -> PatternRule {
    let lhs = GraphBuilder::default()
        .push_h(1)
        .push_cz(0, 1)
        .expect("All qubits should be different")
        .push_h(1)
        .build();

    let rhs = GraphBuilder::default()
        .push_cx(0, 1)
        .expect("All qubits should be different")
        .build();

    PatternRule::new(
        RuleMetadata::new(
            function_name!(),
            "Transforms H CZ H into CX.",
            RuleGroup::BasisChange,
            90,
        ),
        lhs,
        rhs,
    )
    .expect("Built-in rule should be valid")
}

#[named]
pub(crate) fn hadamard_ccx_hadamard() -> PatternRule {
    let lhs = GraphBuilder::default()
        .push_h(2)
        .push_ccx(0, 1, 2)
        .expect("All qubits should be different")
        .push_h(2)
        .build();

    let rhs = GraphBuilder::default()
        .push_ccz(0, 1, 2)
        .expect("All qubits should be different")
        .build();

    PatternRule::new(
        RuleMetadata::new(
            function_name!(),
            "Transforms H CCX H into CCZ.",
            RuleGroup::BasisChange,
            90,
        ),
        lhs,
        rhs,
    )
    .expect("Built-in rule should be valid")
}
