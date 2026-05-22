use std::sync::Arc;

use function_name::named;

use crate::{GraphBuilder, PatternRule, RuleGroup, RuleMetadata, RuleRegistry};

pub(crate) fn register(registry: &mut RuleRegistry) {
    registry.register_all(vec![
        Arc::new(double_hadamard()),
        Arc::new(double_x()),
        Arc::new(double_y()),
        Arc::new(double_z()),
        Arc::new(double_cx()),
        Arc::new(double_cy()),
        Arc::new(double_cz()),
        Arc::new(double_ch()),
        Arc::new(double_swap()),
        Arc::new(double_cswap()),
        Arc::new(double_ccx()),
        Arc::new(double_ccz()),
    ]);
}

#[named]
pub(crate) fn double_hadamard() -> PatternRule {
    let lhs = GraphBuilder::default().push_h(0).push_h(0).build();
    let rhs = GraphBuilder::new(1).build();

    PatternRule::new(
        RuleMetadata::new(
            function_name!(),
            "Removes consecutive Hadamard gates.",
            RuleGroup::Redundancy,
            100,
        ),
        lhs,
        rhs,
    )
    .expect("Built-in rule should be valid")
}

#[named]
pub(crate) fn double_x() -> PatternRule {
    let lhs = GraphBuilder::default().push_x(0).push_x(0).build();
    let rhs = GraphBuilder::new(1).build();

    PatternRule::new(
        RuleMetadata::new(
            function_name!(),
            "Removes consecutive X gates.",
            RuleGroup::Redundancy,
            100,
        ),
        lhs,
        rhs,
    )
    .expect("Built-in rule should be valid")
}

#[named]
pub(crate) fn double_y() -> PatternRule {
    let lhs = GraphBuilder::default().push_y(0).push_y(0).build();
    let rhs = GraphBuilder::new(1).build();

    PatternRule::new(
        RuleMetadata::new(
            function_name!(),
            "Removes consecutive Y gates.",
            RuleGroup::Redundancy,
            100,
        ),
        lhs,
        rhs,
    )
    .expect("Built-in rule should be valid")
}

#[named]
pub(crate) fn double_z() -> PatternRule {
    let lhs = GraphBuilder::default().push_z(0).push_z(0).build();
    let rhs = GraphBuilder::new(1).build();

    PatternRule::new(
        RuleMetadata::new(
            function_name!(),
            "Removes consecutive Z gates.",
            RuleGroup::Redundancy,
            100,
        ),
        lhs,
        rhs,
    )
    .expect("Built-in rule should be valid")
}

#[named]
pub(crate) fn double_cx() -> PatternRule {
    let lhs = GraphBuilder::default()
        .push_cx(0, 1)
        .expect("All qubits should be different")
        .push_cx(0, 1)
        .expect("All qubits should be different")
        .build();

    let rhs = GraphBuilder::new(2).build();

    PatternRule::new(
        RuleMetadata::new(
            function_name!(),
            "Removes consecutive CX gates.",
            RuleGroup::Redundancy,
            100,
        ),
        lhs,
        rhs,
    )
    .expect("Built-in rule should be valid")
}

#[named]
pub(crate) fn double_cy() -> PatternRule {
    let lhs = GraphBuilder::default()
        .push_cy(0, 1)
        .expect("All qubits should be different")
        .push_cy(0, 1)
        .expect("All qubits should be different")
        .build();

    let rhs = GraphBuilder::new(2).build();

    PatternRule::new(
        RuleMetadata::new(
            function_name!(),
            "Removes consecutive CY gates.",
            RuleGroup::Redundancy,
            100,
        ),
        lhs,
        rhs,
    )
    .expect("Built-in rule should be valid")
}

#[named]
pub(crate) fn double_cz() -> PatternRule {
    let lhs = GraphBuilder::default()
        .push_cz(0, 1)
        .expect("All qubits should be different")
        .push_cz(0, 1)
        .expect("All qubits should be different")
        .build();

    let rhs = GraphBuilder::new(2).build();

    PatternRule::new(
        RuleMetadata::new(
            function_name!(),
            "Removes consecutive CZ gates.",
            RuleGroup::Redundancy,
            100,
        ),
        lhs,
        rhs,
    )
    .expect("Built-in rule should be valid")
}

#[named]
pub(crate) fn double_ch() -> PatternRule {
    let lhs = GraphBuilder::default()
        .push_ch(0, 1)
        .expect("All qubits should be different")
        .push_ch(0, 1)
        .expect("All qubits should be different")
        .build();

    let rhs = GraphBuilder::new(2).build();

    PatternRule::new(
        RuleMetadata::new(
            function_name!(),
            "Removes consecutive CH gates.",
            RuleGroup::Redundancy,
            100,
        ),
        lhs,
        rhs,
    )
    .expect("Built-in rule should be valid")
}

#[named]
pub(crate) fn double_swap() -> PatternRule {
    let lhs = GraphBuilder::default()
        .push_swap(0, 1)
        .expect("All qubits should be different")
        .push_swap(0, 1)
        .expect("All qubits should be different")
        .build();

    let rhs = GraphBuilder::new(2).build();

    PatternRule::new(
        RuleMetadata::new(
            function_name!(),
            "Removes consecutive SWAP gates.",
            RuleGroup::Redundancy,
            100,
        ),
        lhs,
        rhs,
    )
    .expect("Built-in rule should be valid")
}

#[named]
pub(crate) fn double_cswap() -> PatternRule {
    let lhs = GraphBuilder::default()
        .push_cswap(0, 1, 2)
        .expect("All qubits should be different")
        .push_cswap(0, 1, 2)
        .expect("All qubits should be different")
        .build();

    let rhs = GraphBuilder::new(3).build();

    PatternRule::new(
        RuleMetadata::new(
            function_name!(),
            "Removes consecutive CSWAP gates.",
            RuleGroup::Redundancy,
            100,
        ),
        lhs,
        rhs,
    )
    .expect("Built-in rule should be valid")
}

#[named]
pub(crate) fn double_ccx() -> PatternRule {
    let lhs = GraphBuilder::default()
        .push_ccx(0, 1, 2)
        .expect("All qubits should be different")
        .push_ccx(0, 1, 2)
        .expect("All qubits should be different")
        .build();

    let rhs = GraphBuilder::new(3).build();

    PatternRule::new(
        RuleMetadata::new(
            function_name!(),
            "Removes consecutive CCX gates.",
            RuleGroup::Redundancy,
            100,
        ),
        lhs,
        rhs,
    )
    .expect("Built-in rule should be valid")
}

#[named]
pub(crate) fn double_ccz() -> PatternRule {
    let lhs = GraphBuilder::default()
        .push_ccz(0, 1, 2)
        .expect("All qubits should be different")
        .push_ccz(0, 1, 2)
        .expect("All qubits should be different")
        .build();

    let rhs = GraphBuilder::new(3).build();

    PatternRule::new(
        RuleMetadata::new(
            function_name!(),
            "Removes consecutive CCZ gates.",
            RuleGroup::Redundancy,
            100,
        ),
        lhs,
        rhs,
    )
    .expect("Built-in rule should be valid")
}
