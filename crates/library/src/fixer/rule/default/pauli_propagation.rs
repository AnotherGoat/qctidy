use std::sync::Arc;

use function_name::named;

use crate::{GraphBuilder, PatternRule, RuleCategory, RuleMetadata, RuleRegistry};

pub(crate) fn register(registry: &mut RuleRegistry) {
    registry.register_all(vec![
        Arc::new(x_on_control_qubit_between_cx()),
        Arc::new(x_on_target_qubit_between_cx()),
        Arc::new(z_on_control_qubit_between_cx()),
        Arc::new(z_on_target_qubit_between_cx()),
        Arc::new(x_between_cz()),
        Arc::new(z_between_cz()),
    ]);
}

#[named]
pub(crate) fn x_on_control_qubit_between_cx() -> PatternRule {
    let lhs = GraphBuilder::default()
        .push_cx(0, 1)
        .expect("All qubits should be different")
        .push_x(0)
        .push_cx(0, 1)
        .expect("All qubits should be different")
        .build();

    let rhs = GraphBuilder::default().push_x(0).push_x(1).build();

    PatternRule::new(
        RuleMetadata::new(
            "Q001",
            function_name!(),
            "X on the control qubit can be propagated between two CX gates",
            RuleCategory::PauliPropagation,
            100,
        ),
        lhs,
        rhs,
    )
    .expect("Built-in rule should be valid")
}

#[named]
pub(crate) fn x_on_target_qubit_between_cx() -> PatternRule {
    let lhs = GraphBuilder::default()
        .push_cx(0, 1)
        .expect("All qubits should be different")
        .push_x(1)
        .push_cx(0, 1)
        .expect("All qubits should be different")
        .build();

    let rhs = GraphBuilder::default().push_x(1).build();

    PatternRule::new(
        RuleMetadata::new(
            "Q002",
            function_name!(),
            "X on the target qubit can be propagated between two CX gates",
            RuleCategory::PauliPropagation,
            100,
        ),
        lhs,
        rhs,
    )
    .expect("Built-in rule should be valid")
}

#[named]
pub(crate) fn z_on_control_qubit_between_cx() -> PatternRule {
    let lhs = GraphBuilder::default()
        .push_cx(0, 1)
        .expect("All qubits should be different")
        .push_z(0)
        .push_cx(0, 1)
        .expect("All qubits should be different")
        .build();

    let rhs = GraphBuilder::new(2).push_z(0).build();

    PatternRule::new(
        RuleMetadata::new(
            "Q003",
            function_name!(),
            "Z on the control qubit can be propagated between two CX gates",
            RuleCategory::PauliPropagation,
            100,
        ),
        lhs,
        rhs,
    )
    .expect("Built-in rule should be valid")
}

#[named]
pub(crate) fn z_on_target_qubit_between_cx() -> PatternRule {
    let lhs = GraphBuilder::default()
        .push_cx(0, 1)
        .expect("All qubits should be different")
        .push_z(1)
        .push_cx(0, 1)
        .expect("All qubits should be different")
        .build();

    let rhs = GraphBuilder::default().push_z(0).push_z(1).build();

    PatternRule::new(
        RuleMetadata::new(
            "Q004",
            function_name!(),
            "Z on the target qubit can be propagated between two CX gates",
            RuleCategory::PauliPropagation,
            100,
        ),
        lhs,
        rhs,
    )
    .expect("Built-in rule should be valid")
}

#[named]
pub(crate) fn x_between_cz() -> PatternRule {
    let lhs = GraphBuilder::default()
        .push_cz(0, 1)
        .expect("All qubits should be different")
        .push_x(0)
        .push_cz(0, 1)
        .expect("All qubits should be different")
        .build();

    let rhs = GraphBuilder::default().push_x(0).push_z(1).build();

    PatternRule::new(
        RuleMetadata::new(
            "Q005",
            function_name!(),
            "X can be propagated between two CZ gates",
            RuleCategory::PauliPropagation,
            100,
        ),
        lhs,
        rhs,
    )
    .expect("Built-in rule should be valid")
}

#[named]
pub(crate) fn z_between_cz() -> PatternRule {
    let lhs = GraphBuilder::default()
        .push_cz(0, 1)
        .expect("All qubits should be different")
        .push_z(0)
        .push_cz(0, 1)
        .expect("All qubits should be different")
        .build();

    let rhs = GraphBuilder::new(2).push_z(0).build();

    PatternRule::new(
        RuleMetadata::new(
            "Q006",
            function_name!(),
            "Z can be propagated between two CZ gates",
            RuleCategory::PauliPropagation,
            100,
        ),
        lhs,
        rhs,
    )
    .expect("Built-in rule should be valid")
}
