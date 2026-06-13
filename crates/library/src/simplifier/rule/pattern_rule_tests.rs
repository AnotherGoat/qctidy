use std::f64::consts::{FRAC_PI_2, PI};

use crate::{
    GateType, Graph, GraphBuilder, PatternRule, Position, RuleBuildError, RuleGroup, RuleMetadata,
};

fn metadata() -> RuleMetadata {
    RuleMetadata::new("test_rule", "Test rule.", RuleGroup::PhaseCompaction, 100)
}

#[test]
fn fails_with_invalid_left_side() {
    let mut lhs = Graph::default();
    lhs.replace_node(
        GateType::X,
        Position::new(0, 0),
        Some(PI),
        Some(0.0_f64),
        Some(FRAC_PI_2),
        Some(1),
    );

    let rhs = GraphBuilder::default().push_x(0).build();

    let result = PatternRule::new(metadata(), lhs, rhs);

    assert!(matches!(result, Err(RuleBuildError::InvalidSide { .. })));
}

#[test]
fn fails_with_invalid_right_side() {
    let lhs = GraphBuilder::default().push_x(0).build();

    let mut rhs = Graph::default();
    rhs.replace_node(
        GateType::X,
        Position::new(0, 0),
        Some(PI),
        Some(0.0_f64),
        Some(FRAC_PI_2),
        Some(1),
    );

    let result = PatternRule::new(metadata(), lhs, rhs);

    assert!(matches!(result, Err(RuleBuildError::InvalidSide { .. })));
}

#[test]
fn fails_with_empty_pattern() {
    let lhs = GraphBuilder::default().build();
    let rhs = GraphBuilder::default().push_x(0).build();

    let result = PatternRule::new(metadata(), lhs, rhs);

    assert!(matches!(result, Err(RuleBuildError::EmptyPattern)));
}

#[test]
fn fails_with_bigger_lhs() {
    let lhs = GraphBuilder::default()
        .push_cx(0, 1)
        .expect("All qubits should be different")
        .build();

    let rhs = GraphBuilder::default().push_x(0).build();

    let result = PatternRule::new(metadata(), lhs, rhs);

    assert!(matches!(
        result,
        Err(RuleBuildError::InvalidHeight {
            lhs_height: 2,
            rhs_height: 1,
        })
    ));
}

#[test]
fn fails_with_bigger_rhs() {
    let lhs = GraphBuilder::default().push_x(0).build();

    let rhs = GraphBuilder::default()
        .push_cx(0, 1)
        .expect("All qubits should be different")
        .build();

    let result = PatternRule::new(metadata(), lhs, rhs);

    assert!(matches!(
        result,
        Err(RuleBuildError::InvalidHeight {
            lhs_height: 1,
            rhs_height: 2,
        })
    ));
}

#[test]
fn fails_with_non_contiguous_pattern() {
    let lhs = GraphBuilder::new(3).push_x(0).push_x(2).build();

    let rhs = GraphBuilder::new(3).push_x(0).build();

    let result = PatternRule::new(metadata(), lhs, rhs);

    assert!(matches!(result, Err(RuleBuildError::NonContiguousPattern)));
}

#[test]
fn fails_with_identical_graphs() {
    let lhs = GraphBuilder::default().push_x(0).build();
    let rhs = GraphBuilder::default().push_x(0).build();

    let result = PatternRule::new(metadata(), lhs, rhs);

    assert!(matches!(result, Err(RuleBuildError::IdenticalGraphs)));
}

#[test]
fn fails_with_non_compacting_rule() {
    let lhs = GraphBuilder::default().push_x(0).push_z(0).build();

    let rhs = GraphBuilder::default().push_h(0).push_s(0).build();

    let result = PatternRule::new(metadata(), lhs, rhs);

    assert!(matches!(
        result,
        Err(RuleBuildError::NonCompactingRule {
            lhs_gate_count: 2,
            rhs_gate_count: 2,
        })
    ));
}

#[test]
fn fails_with_non_equivalent_graphs() {
    let lhs = GraphBuilder::default().push_x(0).push_x(0).build();
    let rhs = GraphBuilder::default().push_z(0).build();

    let result = PatternRule::new(metadata(), lhs, rhs);

    assert!(matches!(result, Err(RuleBuildError::NonEquivalentGraphs)));
}

#[test]
fn creates_valid_rule() {
    let lhs = GraphBuilder::default().push_h(0).push_h(0).build();

    let rhs = GraphBuilder::new(1).build();

    let result = PatternRule::new(metadata(), lhs, rhs);

    result.unwrap();
}
