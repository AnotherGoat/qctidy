use std::sync::Arc;

use function_name::named;

use crate::{GraphBuilder, PatternRule, RuleGroup, RuleMetadata, RuleRegistry};

pub(crate) fn register(registry: &mut RuleRegistry) {
    registry.register_all(vec![
        Arc::new(double_s()),
        Arc::new(triple_s()),
        Arc::new(double_t()),
        Arc::new(quadruple_t()),
        Arc::new(double_s_dagger()),
        Arc::new(triple_s_dagger()),
        Arc::new(double_t_dagger()),
        Arc::new(quadruple_t_dagger()),
        Arc::new(s_followed_by_dagger()),
        Arc::new(s_preceded_by_dagger()),
        Arc::new(t_followed_by_dagger()),
        Arc::new(t_preceded_by_dagger()),
        Arc::new(z_followed_by_s()),
        Arc::new(z_preceded_by_s()),
        Arc::new(z_followed_by_s_dagger()),
        Arc::new(z_preceded_by_s_dagger()),
        Arc::new(y_s_x_to_s()),
        Arc::new(x_s_y_to_s()),
        Arc::new(y_s_dagger_x_to_s_dagger()),
        Arc::new(x_s_dagger_y_to_s_dagger()),
    ]);
}

#[named]
pub(crate) fn double_s() -> PatternRule {
    let lhs = GraphBuilder::default().push_s(0).push_s(0).build();
    let rhs = GraphBuilder::default().push_z(0).build();

    PatternRule::new(
        RuleMetadata::new(
            function_name!(),
            "Compacts consecutive S gates: S S => Z.",
            RuleGroup::PhaseCompaction,
            100,
        ),
        lhs,
        rhs,
    )
    .expect("Built-in rule should be valid")
}

#[named]
pub(crate) fn triple_s() -> PatternRule {
    let lhs = GraphBuilder::default()
        .push_s(0)
        .push_s(0)
        .push_s(0)
        .build();

    let rhs = GraphBuilder::default().push_sdg(0).build();

    PatternRule::new(
        RuleMetadata::new(
            function_name!(),
            "Compacts three consecutive S gates: S S S => Sdg.",
            RuleGroup::PhaseCompaction,
            100,
        ),
        lhs,
        rhs,
    )
    .expect("Built-in rule should be valid")
}

#[named]
pub(crate) fn double_t() -> PatternRule {
    let lhs = GraphBuilder::default().push_t(0).push_t(0).build();
    let rhs = GraphBuilder::default().push_s(0).build();

    PatternRule::new(
        RuleMetadata::new(
            function_name!(),
            "Compacts consecutive T gates: T T => S.",
            RuleGroup::PhaseCompaction,
            100,
        ),
        lhs,
        rhs,
    )
    .expect("Built-in rule should be valid")
}

#[named]
pub(crate) fn quadruple_t() -> PatternRule {
    let lhs = GraphBuilder::default()
        .push_t(0)
        .push_t(0)
        .push_t(0)
        .push_t(0)
        .build();

    let rhs = GraphBuilder::default().push_z(0).build();

    PatternRule::new(
        RuleMetadata::new(
            function_name!(),
            "Compacts four consecutive T gates: T T T T => Z.",
            RuleGroup::PhaseCompaction,
            100,
        ),
        lhs,
        rhs,
    )
    .expect("Built-in rule should be valid")
}

#[named]
pub(crate) fn double_s_dagger() -> PatternRule {
    let lhs = GraphBuilder::default().push_sdg(0).push_sdg(0).build();
    let rhs = GraphBuilder::default().push_z(0).build();

    PatternRule::new(
        RuleMetadata::new(
            function_name!(),
            "Compacts consecutive S daggers: Sdg Sdg => Z.",
            RuleGroup::PhaseCompaction,
            100,
        ),
        lhs,
        rhs,
    )
    .expect("Built-in rule should be valid")
}

#[named]
pub(crate) fn triple_s_dagger() -> PatternRule {
    let lhs = GraphBuilder::default()
        .push_sdg(0)
        .push_sdg(0)
        .push_sdg(0)
        .build();

    let rhs = GraphBuilder::default().push_s(0).build();

    PatternRule::new(
        RuleMetadata::new(
            function_name!(),
            "Compacts three consecutive S daggers: Sdg Sdg Sdg => S.",
            RuleGroup::PhaseCompaction,
            100,
        ),
        lhs,
        rhs,
    )
    .expect("Built-in rule should be valid")
}

#[named]
pub(crate) fn double_t_dagger() -> PatternRule {
    let lhs = GraphBuilder::default().push_tdg(0).push_tdg(0).build();
    let rhs = GraphBuilder::default().push_sdg(0).build();

    PatternRule::new(
        RuleMetadata::new(
            function_name!(),
            "Compacts consecutive T daggers: Tdg Tdg => Sdg.",
            RuleGroup::PhaseCompaction,
            100,
        ),
        lhs,
        rhs,
    )
    .expect("Built-in rule should be valid")
}

#[named]
pub(crate) fn quadruple_t_dagger() -> PatternRule {
    let lhs = GraphBuilder::default()
        .push_tdg(0)
        .push_tdg(0)
        .push_tdg(0)
        .push_tdg(0)
        .build();

    let rhs = GraphBuilder::default().push_z(0).build();

    PatternRule::new(
        RuleMetadata::new(
            function_name!(),
            "Compacts four consecutive T daggers: Tdg Tdg Tdg Tdg => Z.",
            RuleGroup::PhaseCompaction,
            100,
        ),
        lhs,
        rhs,
    )
    .expect("Built-in rule should be valid")
}

#[named]
pub(crate) fn s_followed_by_dagger() -> PatternRule {
    let lhs = GraphBuilder::default().push_s(0).push_sdg(0).build();
    let rhs = GraphBuilder::new(1).build();

    PatternRule::new(
        RuleMetadata::new(
            function_name!(),
            "Removes S gates followed by their dagger.",
            RuleGroup::PhaseCompaction,
            100,
        ),
        lhs,
        rhs,
    )
    .expect("Built-in rule should be valid")
}

#[named]
pub(crate) fn s_preceded_by_dagger() -> PatternRule {
    let lhs = GraphBuilder::default().push_sdg(0).push_s(0).build();
    let rhs = GraphBuilder::new(1).build();

    PatternRule::new(
        RuleMetadata::new(
            function_name!(),
            "Removes S gates preceded by their dagger.",
            RuleGroup::PhaseCompaction,
            90,
        ),
        lhs,
        rhs,
    )
    .expect("Built-in rule should be valid")
}

#[named]
pub(crate) fn t_followed_by_dagger() -> PatternRule {
    let lhs = GraphBuilder::default().push_t(0).push_tdg(0).build();
    let rhs = GraphBuilder::new(1).build();

    PatternRule::new(
        RuleMetadata::new(
            function_name!(),
            "Removes T gates followed by their dagger.",
            RuleGroup::PhaseCompaction,
            100,
        ),
        lhs,
        rhs,
    )
    .expect("Built-in rule should be valid")
}

#[named]
pub(crate) fn t_preceded_by_dagger() -> PatternRule {
    let lhs = GraphBuilder::default().push_tdg(0).push_t(0).build();
    let rhs = GraphBuilder::new(1).build();

    PatternRule::new(
        RuleMetadata::new(
            function_name!(),
            "Removes T gates preceded by their dagger.",
            RuleGroup::PhaseCompaction,
            90,
        ),
        lhs,
        rhs,
    )
    .expect("Built-in rule should be valid")
}

#[named]
pub(crate) fn z_followed_by_s() -> PatternRule {
    let lhs = GraphBuilder::default().push_z(0).push_s(0).build();
    let rhs = GraphBuilder::default().push_sdg(0).build();

    PatternRule::new(
        RuleMetadata::new(
            function_name!(),
            "Compacts Z gates followed by S gates: Z S => Sdg.",
            RuleGroup::PhaseCompaction,
            100,
        ),
        lhs,
        rhs,
    )
    .expect("Built-in rule should be valid")
}

#[named]
pub(crate) fn z_preceded_by_s() -> PatternRule {
    let lhs = GraphBuilder::default().push_s(0).push_z(0).build();
    let rhs = GraphBuilder::default().push_sdg(0).build();

    PatternRule::new(
        RuleMetadata::new(
            function_name!(),
            "Compacts Z gates preceded by S gates: S Z => Sdg.",
            RuleGroup::PhaseCompaction,
            90,
        ),
        lhs,
        rhs,
    )
    .expect("Built-in rule should be valid")
}

#[named]
pub(crate) fn z_followed_by_s_dagger() -> PatternRule {
    let lhs = GraphBuilder::default().push_z(0).push_sdg(0).build();
    let rhs = GraphBuilder::default().push_s(0).build();

    PatternRule::new(
        RuleMetadata::new(
            function_name!(),
            "Compacts Z gates followed by S daggers: Z Sdg => S.",
            RuleGroup::PhaseCompaction,
            100,
        ),
        lhs,
        rhs,
    )
    .expect("Built-in rule should be valid")
}

#[named]
pub(crate) fn z_preceded_by_s_dagger() -> PatternRule {
    let lhs = GraphBuilder::default().push_sdg(0).push_z(0).build();
    let rhs = GraphBuilder::default().push_s(0).build();

    PatternRule::new(
        RuleMetadata::new(
            function_name!(),
            "Compacts Z gates preceded by S daggers: Sdg Z => S.",
            RuleGroup::PhaseCompaction,
            90,
        ),
        lhs,
        rhs,
    )
    .expect("Built-in rule should be valid")
}

#[named]
pub(crate) fn y_s_x_to_s() -> PatternRule {
    let lhs = GraphBuilder::default()
        .push_y(0)
        .push_s(0)
        .push_x(0)
        .build();

    let rhs = GraphBuilder::default().push_s(0).build();

    PatternRule::new(
        RuleMetadata::new(
            function_name!(),
            "Compacts Y S X into S.",
            RuleGroup::PhaseCompaction,
            100,
        ),
        lhs,
        rhs,
    )
    .expect("Built-in rule should be valid")
}

#[named]
pub(crate) fn x_s_y_to_s() -> PatternRule {
    let lhs = GraphBuilder::default()
        .push_x(0)
        .push_s(0)
        .push_y(0)
        .build();

    let rhs = GraphBuilder::default().push_s(0).build();

    PatternRule::new(
        RuleMetadata::new(
            function_name!(),
            "Compacts X S Y into S.",
            RuleGroup::PhaseCompaction,
            100,
        ),
        lhs,
        rhs,
    )
    .expect("Built-in rule should be valid")
}

#[named]
pub(crate) fn y_s_dagger_x_to_s_dagger() -> PatternRule {
    let lhs = GraphBuilder::default()
        .push_y(0)
        .push_sdg(0)
        .push_x(0)
        .build();

    let rhs = GraphBuilder::default().push_sdg(0).build();

    PatternRule::new(
        RuleMetadata::new(
            function_name!(),
            "Compacts Y Sdg X into Sdg.",
            RuleGroup::PhaseCompaction,
            100,
        ),
        lhs,
        rhs,
    )
    .expect("Built-in rule should be valid")
}

#[named]
pub(crate) fn x_s_dagger_y_to_s_dagger() -> PatternRule {
    let lhs = GraphBuilder::default()
        .push_x(0)
        .push_sdg(0)
        .push_y(0)
        .build();

    let rhs = GraphBuilder::default().push_sdg(0).build();

    PatternRule::new(
        RuleMetadata::new(
            function_name!(),
            "Compacts X Sdg Y into Sdg.",
            RuleGroup::PhaseCompaction,
            100,
        ),
        lhs,
        rhs,
    )
    .expect("Built-in rule should be valid")
}
