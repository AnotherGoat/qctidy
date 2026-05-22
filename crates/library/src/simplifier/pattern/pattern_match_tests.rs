use crate::{
    Graph, GraphBuilder,
    simplifier::{
        pattern::{matcher, pattern_match_asserter::PatternMatchAsserter},
        rule::default::redundancy,
    },
};

#[test]
fn match_empty_graph_finds_nothing() {
    let graph = Graph::default();
    let rule = redundancy::double_hadamard();

    let matches = matcher::find_matches(&graph, &rule);

    assert!(matches.is_empty());
}

#[test]
fn match_double_hadamard_on_single_qubit() {
    let graph = GraphBuilder::default().push_h(0).push_h(0).build();
    let rule = redundancy::double_hadamard();

    let matches = matcher::find_matches(&graph, &rule);

    assert_eq!(matches.len(), 1);
    PatternMatchAsserter::new(&matches[0]).single_qubit_double_gate("double_hadamard");
}

#[test]
fn match_double_x_on_single_qubit() {
    let graph = GraphBuilder::default().push_x(0).push_x(0).build();
    let rule = redundancy::double_x();

    let matches = matcher::find_matches(&graph, &rule);

    assert_eq!(matches.len(), 1);
    PatternMatchAsserter::new(&matches[0]).single_qubit_double_gate("double_x");
}

#[test]
fn match_double_y_on_single_qubit() {
    let graph = GraphBuilder::default().push_y(0).push_y(0).build();
    let rule = redundancy::double_y();

    let matches = matcher::find_matches(&graph, &rule);

    assert_eq!(matches.len(), 1);
    PatternMatchAsserter::new(&matches[0]).single_qubit_double_gate("double_y");
}

#[test]
fn match_double_z_on_single_qubit() {
    let graph = GraphBuilder::default().push_z(0).push_z(0).build();
    let rule = redundancy::double_z();

    let matches = matcher::find_matches(&graph, &rule);

    assert_eq!(matches.len(), 1);
    PatternMatchAsserter::new(&matches[0]).single_qubit_double_gate("double_z");
}

#[test]
fn match_double_cx_on_two_qubits() {
    let graph = GraphBuilder::default()
        .push_cx(0, 1)
        .unwrap()
        .push_cx(0, 1)
        .unwrap()
        .build();
    let rule = redundancy::double_cx();

    let matches = matcher::find_matches(&graph, &rule);

    assert_eq!(matches.len(), 1);
    PatternMatchAsserter::new(&matches[0]).two_qubit_double_gate("double_cx");
}

#[test]
fn match_double_cy_on_two_qubits() {
    let graph = GraphBuilder::default()
        .push_cy(0, 1)
        .unwrap()
        .push_cy(0, 1)
        .unwrap()
        .build();
    let rule = redundancy::double_cy();

    let matches = matcher::find_matches(&graph, &rule);

    assert_eq!(matches.len(), 1);
    PatternMatchAsserter::new(&matches[0]).two_qubit_double_gate("double_cy");
}

#[test]
fn match_double_cz_on_two_qubits() {
    let graph = GraphBuilder::default()
        .push_cz(0, 1)
        .unwrap()
        .push_cz(0, 1)
        .unwrap()
        .build();
    let rule = redundancy::double_cz();

    let matches = matcher::find_matches(&graph, &rule);

    assert_eq!(matches.len(), 1);
    PatternMatchAsserter::new(&matches[0]).two_qubit_double_gate("double_cz");
}

#[test]
fn match_double_ch_on_two_qubits() {
    let graph = GraphBuilder::default()
        .push_ch(0, 1)
        .unwrap()
        .push_ch(0, 1)
        .unwrap()
        .build();
    let rule = redundancy::double_ch();

    let matches = matcher::find_matches(&graph, &rule);

    assert_eq!(matches.len(), 1);
    PatternMatchAsserter::new(&matches[0]).two_qubit_double_gate("double_ch");
}

#[test]
fn match_double_swap_on_two_qubits() {
    let graph = GraphBuilder::default()
        .push_swap(0, 1)
        .unwrap()
        .push_swap(0, 1)
        .unwrap()
        .build();
    let rule = redundancy::double_swap();

    let matches = matcher::find_matches(&graph, &rule);

    assert_eq!(matches.len(), 1);
    PatternMatchAsserter::new(&matches[0]).two_qubit_double_gate("double_swap");
}

#[test]
fn match_double_cswap_on_three_qubits() {
    let graph = GraphBuilder::default()
        .push_cswap(0, 1, 2)
        .unwrap()
        .push_cswap(0, 1, 2)
        .unwrap()
        .build();
    let rule = redundancy::double_cswap();

    let matches = matcher::find_matches(&graph, &rule);

    assert_eq!(matches.len(), 1);
    PatternMatchAsserter::new(&matches[0]).three_qubit_double_gate("double_cswap");
}

#[test]
fn match_double_ccx_on_three_qubits() {
    let graph = GraphBuilder::default()
        .push_ccx(0, 1, 2)
        .unwrap()
        .push_ccx(0, 1, 2)
        .unwrap()
        .build();
    let rule = redundancy::double_ccx();

    let matches = matcher::find_matches(&graph, &rule);

    assert_eq!(matches.len(), 1);
    PatternMatchAsserter::new(&matches[0]).three_qubit_double_gate("double_ccx");
}

#[test]
fn match_double_ccz_on_three_qubits() {
    let graph = GraphBuilder::default()
        .push_ccz(0, 1, 2)
        .unwrap()
        .push_ccz(0, 1, 2)
        .unwrap()
        .build();
    let rule = redundancy::double_ccz();

    let matches = matcher::find_matches(&graph, &rule);

    assert_eq!(matches.len(), 1);
    PatternMatchAsserter::new(&matches[0]).three_qubit_double_gate("double_ccz");
}

#[test]
fn match_rule_multiple_times() {
    let graph = GraphBuilder::default()
        .push_h(0)
        .push_h(0)
        .push_x(0)
        .push_h(0)
        .push_h(0)
        .build();
    let rule = redundancy::double_hadamard();

    let matches = matcher::find_matches(&graph, &rule);

    assert_eq!(matches.len(), 2);

    for r#match in &matches {
        PatternMatchAsserter::new(r#match)
            .rule_id("double_hadamard")
            .covered_count(2);
    }
}

#[test]
fn match_rule_on_multiple_qubits() {
    let graph = GraphBuilder::default()
        .push_h(0)
        .push_h(0)
        .push_x(1)
        .push_h(2)
        .push_h(2)
        .build();
    let rule = redundancy::double_hadamard();

    let matches = matcher::find_matches(&graph, &rule);

    assert_eq!(matches.len(), 2);

    for r#match in &matches {
        PatternMatchAsserter::new(r#match)
            .rule_id("double_hadamard")
            .covered_count(2);
    }
}

#[test]
fn match_rule_without_repeated_gates() {
    let graph = GraphBuilder::default()
        .push_h(0)
        .push_h(0)
        .push_h(0)
        .push_h(0)
        .push_h(0)
        .build();
    let rule = redundancy::double_hadamard();

    let matches = matcher::find_matches(&graph, &rule);

    assert_eq!(matches.len(), 2);

    for r#match in &matches {
        PatternMatchAsserter::new(r#match)
            .rule_id("double_hadamard")
            .covered_count(2);
    }
}

#[test]
fn dont_match_asymmetrical_controlled_gate() {
    let graph = GraphBuilder::default()
        .push_cx(0, 1)
        .unwrap()
        .push_cx(1, 0)
        .unwrap()
        .build();

    let rule = redundancy::double_cx();

    let matches = matcher::find_matches(&graph, &rule);

    assert!(matches.is_empty());
}

#[test]
fn match_symmetrical_controlled_gate() {
    let graph = GraphBuilder::default()
        .push_cz(0, 1)
        .unwrap()
        .push_cz(1, 0)
        .unwrap()
        .build();

    let rule = redundancy::double_cz();

    let matches = matcher::find_matches(&graph, &rule);

    assert_eq!(matches.len(), 1);
    PatternMatchAsserter::new(&matches[0]).two_qubit_double_gate("double_cz");
}
