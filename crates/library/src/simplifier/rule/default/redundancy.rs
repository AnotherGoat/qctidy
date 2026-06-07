use std::sync::Arc;

use function_name::named;

use crate::{GraphBuilder, PatternRule, RuleGroup, RuleMetadata, RuleRegistry};

pub(crate) fn register(registry: &mut RuleRegistry) {
    registry.register_all(vec![
        Arc::new(remove_identity()),
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
        Arc::new(null_phase()),
        Arc::new(null_rx()),
        Arc::new(null_ry()),
        Arc::new(null_rz()),
        Arc::new(null_cp()),
    ]);
}

#[named]
pub(crate) fn remove_identity() -> PatternRule {
    let lhs = GraphBuilder::default().push_id(0).build();
    let rhs = GraphBuilder::new(1).build();

    PatternRule::new(
        RuleMetadata::new(
            function_name!(),
            "Removes identity gates.",
            RuleGroup::Redundancy,
            100,
        ),
        lhs,
        rhs,
    )
    .expect("Built-in rule {} should be valid")
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

#[named]
pub(crate) fn null_phase() -> PatternRule {
    let lhs = GraphBuilder::default()
        .push_p(0.0, 0)
        .expect("Angle should be valid")
        .build();
    let rhs = GraphBuilder::new(1).build();

    PatternRule::new(
        RuleMetadata::new(
            function_name!(),
            "Removes phase gates that have no phase angle.",
            RuleGroup::Redundancy,
            100,
        ),
        lhs,
        rhs,
    )
    .expect("Built-in rule should be valid")
}

#[named]
pub(crate) fn null_rx() -> PatternRule {
    let lhs = GraphBuilder::default()
        .push_rx(0.0, 0)
        .expect("Angle should be valid")
        .build();
    let rhs = GraphBuilder::new(1).build();

    PatternRule::new(
        RuleMetadata::new(
            function_name!(),
            "Removes RX gates that have no rotation angle.",
            RuleGroup::Redundancy,
            100,
        ),
        lhs,
        rhs,
    )
    .expect("Built-in rule should be valid")
}

#[named]
pub(crate) fn null_ry() -> PatternRule {
    let lhs = GraphBuilder::default()
        .push_ry(0.0, 0)
        .expect("Angle should be valid")
        .build();
    let rhs = GraphBuilder::new(1).build();

    PatternRule::new(
        RuleMetadata::new(
            function_name!(),
            "Removes RY gates that have no rotation angle.",
            RuleGroup::Redundancy,
            100,
        ),
        lhs,
        rhs,
    )
    .expect("Built-in rule should be valid")
}

#[named]
pub(crate) fn null_rz() -> PatternRule {
    let lhs = GraphBuilder::default()
        .push_rz(0.0, 0)
        .expect("Angle should be valid")
        .build();
    let rhs = GraphBuilder::new(1).build();

    PatternRule::new(
        RuleMetadata::new(
            function_name!(),
            "Removes RZ gates that have no rotation angle.",
            RuleGroup::Redundancy,
            100,
        ),
        lhs,
        rhs,
    )
    .expect("Built-in rule should be valid")
}

#[named]
pub(crate) fn null_cp() -> PatternRule {
    let lhs = GraphBuilder::default()
        .push_cp(0.0, 0, 1)
        .expect("Angle should be valid and all qubits should be different")
        .build();
    let rhs = GraphBuilder::new(2).build();

    PatternRule::new(
        RuleMetadata::new(
            function_name!(),
            "Removes CP gates that have no phase angle.",
            RuleGroup::Redundancy,
            100,
        ),
        lhs,
        rhs,
    )
    .expect("Built-in rule should be valid")
}
