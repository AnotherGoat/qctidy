use std::sync::Arc;

use function_name::named;

use crate::{GraphBuilder, PatternRule, RuleCategory, RuleMetadata, RuleRegistry};

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
            "P001",
            function_name!(),
            "Consecutive S gates are equivalent to Z",
            RuleCategory::PhaseCompaction,
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
            "P002",
            function_name!(),
            "Three consecutive S gates are equivalent to Sdg",
            RuleCategory::PhaseCompaction,
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
            "P003",
            function_name!(),
            "Consecutive T gates are equivalent to S",
            RuleCategory::PhaseCompaction,
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
            "P004",
            function_name!(),
            "Four consecutive T gates are equivalent to Z",
            RuleCategory::PhaseCompaction,
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
            "P005",
            function_name!(),
            "Consecutive Sdg gates are equivalent to Z",
            RuleCategory::PhaseCompaction,
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
            "P006",
            function_name!(),
            "Three consecutive Sdg gates are equivalent to S",
            RuleCategory::PhaseCompaction,
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
            "P007",
            function_name!(),
            "Consecutive Tdg gates are equivalent to Sdg",
            RuleCategory::PhaseCompaction,
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
            "P008",
            function_name!(),
            "Four consecutive Tdg gates are equivalent to Z",
            RuleCategory::PhaseCompaction,
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
            "P009",
            function_name!(),
            "S gate followed by its dagger is redundant",
            RuleCategory::PhaseCompaction,
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
            "P010",
            function_name!(),
            "S gate preceded by its dagger is redundant",
            RuleCategory::PhaseCompaction,
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
            "P011",
            function_name!(),
            "T gate followed by its dagger is redundant",
            RuleCategory::PhaseCompaction,
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
            "P012",
            function_name!(),
            "T gate preceded by its dagger is redundant",
            RuleCategory::PhaseCompaction,
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
            "P013",
            function_name!(),
            "Z followed by S is equivalent to Sdg",
            RuleCategory::PhaseCompaction,
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
            "P014",
            function_name!(),
            "S followed by Z is equivalent to Sdg",
            RuleCategory::PhaseCompaction,
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
            "P015",
            function_name!(),
            "Z followed by Sdg is equivalent to S",
            RuleCategory::PhaseCompaction,
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
            "P016",
            function_name!(),
            "Sdg followed by Z is equivalent to S",
            RuleCategory::PhaseCompaction,
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
            "P017",
            function_name!(),
            "Y S X is equivalent to S",
            RuleCategory::PhaseCompaction,
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
            "P018",
            function_name!(),
            "X S Y is equivalent to S",
            RuleCategory::PhaseCompaction,
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
            "P019",
            function_name!(),
            "Y Sdg X is equivalent to Sdg",
            RuleCategory::PhaseCompaction,
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
            "P020",
            function_name!(),
            "X Sdg Y is equivalent to Sdg",
            RuleCategory::PhaseCompaction,
            100,
        ),
        lhs,
        rhs,
    )
    .expect("Built-in rule should be valid")
}
