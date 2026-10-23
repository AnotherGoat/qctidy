use crate::{
    GateOperation, GateType, GraphBuilder, Position, RuleConfiguration, RuleSeverity, fixer,
    fixer::fixer_mother,
};

#[test]
fn default_rules_are_ordered_by_code() {
    let rules = fixer::default_rules();

    assert!(rules.len() > 1);
    assert!(
        rules
            .windows(2)
            .all(|pair| *pair[0].code() <= *pair[1].code())
    );
    assert!(
        rules
            .iter()
            .any(|metadata| *metadata.name() == "double_hadamard")
    );
}

#[test]
fn configuration_selects_by_rule_or_category_code() {
    let rules = fixer::default_rules();
    let redundancy = rules
        .iter()
        .find(|metadata| metadata.category().code() == "R")
        .unwrap();
    let other = rules
        .iter()
        .find(|metadata| metadata.category().code() != "R")
        .unwrap();

    let mut configuration = RuleConfiguration::new(RuleSeverity::Warn);
    configuration.ignore("R");

    assert_eq!(configuration.severity(redundancy), RuleSeverity::Off);
    assert_eq!(configuration.severity(other), RuleSeverity::Warn);

    // A rule-specific selection wins over the category override.
    configuration.select(redundancy.code());

    assert_eq!(configuration.severity(redundancy), RuleSeverity::Warn);
}

#[test]
fn detect_reports_diagnostics_without_modifying_the_graph() {
    let graph = GraphBuilder::new(1).push_h(0).push_h(0).build();

    let diagnostics = fixer::detect(&graph, &RuleConfiguration::new(RuleSeverity::Warn));

    let hadamard = diagnostics
        .iter()
        .find(|diagnostic| *diagnostic.metadata().name() == "double_hadamard")
        .expect("double_hadamard should be detected");

    assert_eq!(
        hadamard.positions().as_slice(),
        &[Position::new(0, 0), Position::new(0, 1)]
    );

    assert_eq!(graph.iter_nodes().count(), 2);
}

#[test]
fn detect_reports_the_configured_severity() {
    let graph = GraphBuilder::new(1).push_h(0).push_h(0).build();

    let diagnostics = fixer::detect(&graph, &RuleConfiguration::new(RuleSeverity::Error));

    let hadamard = diagnostics
        .iter()
        .find(|diagnostic| *diagnostic.metadata().name() == "double_hadamard")
        .expect("double_hadamard should be detected");

    assert_eq!(hadamard.severity(), RuleSeverity::Error);
}

#[test]
fn angles_very_close_to_zero_are_removed() {
    let mut graph = GraphBuilder::new(2)
        .push_p(0.0, 0)
        .unwrap()
        .push_rx(1e-8, 0)
        .unwrap()
        .push_ry(1e-9, 0)
        .unwrap()
        .push_rz(-1e-8, 0)
        .unwrap()
        .push_cp(-1e-9, 0, 1)
        .unwrap()
        .build();

    fixer_mother::default().fix(&mut graph, 10);

    let nodes: Vec<_> = graph.iter_nodes_ordered_by_time_step().collect();
    assert_eq!(nodes.len(), 0);
}

#[test]
fn angles_close_to_zero_are_kept() {
    let mut graph = GraphBuilder::new(2)
        .push_p(1.0, 0)
        .unwrap()
        .push_rx(1e-7, 0)
        .unwrap()
        .push_ry(1e-6, 0)
        .unwrap()
        .push_rz(-1e-7, 0)
        .unwrap()
        .push_cp(-1e-6, 0, 1)
        .unwrap()
        .build();

    fixer_mother::default().fix(&mut graph, 10);

    assert_eq!(graph.iter_nodes_ordered_by_time_step().count(), 6);
}

#[test]
fn x_on_control_between_cx_with_identities_compacts_and_propagates() {
    let mut graph = GraphBuilder::new(2)
        .push_operation(&GateOperation::try_cx(0, 1).unwrap())
        .push_operation(&GateOperation::x(0))
        .push_operation(&GateOperation::id(0))
        .push_operation(&GateOperation::id(1))
        .push_operation(&GateOperation::try_cx(0, 1).unwrap())
        .build();

    fixer_mother::default().fix(&mut graph, 10);

    let mut nodes: Vec<_> = graph
        .iter_nodes_ordered_by_qubit()
        .map(|node| (node.position().qubit(), node.r#type()))
        .collect();
    nodes.sort_by_key(|(qubit, _)| *qubit);

    assert_eq!(nodes.len(), 2);
    assert_eq!(nodes[0], (0, GateType::X));
    assert_eq!(nodes[1], (1, GateType::X));
}

#[test]
fn double_x_with_gap_compacts_and_cancels() {
    let mut graph = GraphBuilder::new(1)
        .push_operation(&GateOperation::x(0))
        .push_operation(&GateOperation::id(0))
        .push_operation(&GateOperation::x(0))
        .build();

    fixer_mother::default().fix(&mut graph, 10);

    assert_eq!(graph.iter_nodes_ordered_by_time_step().count(), 0);
}

#[test]
fn fix_preserves_matrix_equivalence() {
    let mut any_changed = false;

    for seed in 0..40 {
        let graph = crate::random::generate(seed, 20, 4);
        let fixed = fixer::fix(graph.clone(), 10);

        assert!(
            fixed.validate().is_ok(),
            "seed {seed} produced an invalid graph"
        );
        assert!(
            fixer::matrix_calculator::are_graphs_equivalent(&graph, &fixed),
            "seed {seed} changed the unitary"
        );

        if fixed != graph {
            any_changed = true;
        }
    }

    assert!(any_changed, "the generated circuits never triggered a fix");
}

#[test]
fn apply_applies_only_the_selected_match() {
    // Two disjoint `CX CX` patterns: one on qubits 0-1, one on qubits 2-3.
    let mut builder = GraphBuilder::new(4);
    builder.push_cx(0, 1).expect("distinct qubits");
    builder.push_cx(0, 1).expect("distinct qubits");
    builder.push_cx(2, 3).expect("distinct qubits");
    builder.push_cx(2, 3).expect("distinct qubits");
    let mut graph = builder.build();

    let diagnostics = fixer::detect(&graph, &RuleConfiguration::new(RuleSeverity::Warn));
    let double_cx: Vec<_> = diagnostics
        .iter()
        .filter(|diagnostic| *diagnostic.metadata().code() == "R006")
        .collect();
    assert_eq!(double_cx.len(), 2);

    // Applying the first match must only remove the pattern on qubits 0-1.
    let replacements = fixer::apply(&mut graph, double_cx[0]).expect("the match should apply");

    let mut expected_builder = GraphBuilder::new(4);
    expected_builder.push_cx(2, 3).expect("distinct qubits");
    expected_builder.push_cx(2, 3).expect("distinct qubits");
    let expected = expected_builder.build();
    assert_eq!(graph, expected);

    // `CX CX => ()` replaces the pair with nothing.
    assert!(replacements.is_empty());
}

#[test]
fn apply_returns_the_replacement_operations() {
    // `S S => Z` (P001): the two S gates collapse into a single Z.
    let mut graph = GraphBuilder::new(1).push_s(0).push_s(0).build();

    let diagnostics = fixer::detect(&graph, &RuleConfiguration::new(RuleSeverity::Warn));
    let double_s = diagnostics
        .iter()
        .find(|diagnostic| *diagnostic.metadata().code() == "P001")
        .expect("P001 should be detected");

    let replacements = fixer::apply(&mut graph, double_s).expect("the match should apply");

    assert_eq!(replacements.len(), 1);
    assert!(matches!(replacements[0], GateOperation::Z { qubit: 0 }));
    assert_eq!(graph, GraphBuilder::new(1).push_z(0).build());
}

#[test]
fn apply_returns_none_when_the_match_is_gone() {
    let mut graph = GraphBuilder::new(1).push_x(0).push_x(0).build();

    let diagnostics = fixer::detect(&graph, &RuleConfiguration::new(RuleSeverity::Warn));
    let double_x = diagnostics
        .iter()
        .find(|diagnostic| *diagnostic.metadata().code() == "R003")
        .expect("R003 should be detected");

    assert!(fixer::apply(&mut graph, double_x).is_some());
    assert!(fixer::apply(&mut graph, double_x).is_none());
}

#[test]
fn apply_returns_none_for_canonicalization_rules() {
    let mut graph = GraphBuilder::new(3).push_x(0).push_x(2).build();

    let diagnostics = fixer::detect(&graph, &RuleConfiguration::new(RuleSeverity::Warn));
    let canonicalization = diagnostics
        .iter()
        .find(|diagnostic| diagnostic.metadata().category().code() == "C")
        .expect("a canonicalization diagnostic should be detected");

    assert!(fixer::apply(&mut graph, canonicalization).is_none());
    assert_eq!(graph, GraphBuilder::new(3).push_x(0).push_x(2).build());
}
