use crate::{
    Graph,
    simplifier::{pattern::matcher, rule::default::redundancy},
};

#[test]
fn match_empty_graph_finds_nothing() {
    let graph = Graph::default();
    let rule = redundancy::double_hadamard();

    let matches = matcher::find_matches(&graph, &rule);

    assert!(matches.is_empty());
}
