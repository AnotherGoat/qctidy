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
fn detect_reports_issues_without_modifying_the_graph() {
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

    let nodes: Vec<_> = graph.iter_nodes_ordered_by_column().collect();
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

    assert_eq!(graph.iter_nodes_ordered_by_column().count(), 6);
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
        .iter_nodes_ordered_by_row()
        .map(|node| (node.position().row(), node.r#type()))
        .collect();
    nodes.sort_by_key(|(row, _)| *row);

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

    assert_eq!(graph.iter_nodes_ordered_by_column().count(), 0);
}
