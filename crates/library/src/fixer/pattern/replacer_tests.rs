use crate::{
    GateType, Graph, GraphBuilder, PatternRule, Position, Rule,
    fixer::rule::{
        default::{control_reversal, redundancy},
        registry::DEFAULT_RULE_REGISTRY,
    },
};

fn add_connected_node(graph: &mut Graph, gate: GateType, position: Position) {
    graph
        .add_node(gate, position, None, None, None, None)
        .expect("The node position should be free");
    graph
        .connect_row_neighbors(position)
        .expect("The node was just added");
}

#[test]
fn replace_default_pattern_rules() {
    let registry = &DEFAULT_RULE_REGISTRY;

    for rule in registry.iter() {
        if let Some(pattern_rule) = rule.as_any().downcast_ref::<PatternRule>() {
            let mut lhs = pattern_rule.lhs().clone();
            let rhs = pattern_rule.rhs();

            let changed = pattern_rule.apply(&mut lhs);
            assert!(changed);
            assert_eq!(lhs, *rhs);
        }
    }
}

#[test]
fn replace_default_pattern_rules_at_offsets() {
    let registry = &DEFAULT_RULE_REGISTRY;

    for rule in registry.iter() {
        let Some(pattern_rule) = rule.as_any().downcast_ref::<PatternRule>() else {
            continue;
        };

        for (row_offset, column_offset) in [(0, 0), (1, 0), (0, 1), (2, 3), (3, 2)] {
            let mut graph = shift(pattern_rule.lhs(), row_offset, column_offset);
            let expected = shift(pattern_rule.rhs(), row_offset, column_offset);

            let changed = pattern_rule.apply(&mut graph);

            assert!(
                changed,
                "{} did not match at ({row_offset}, {column_offset})",
                pattern_rule.metadata().name()
            );
            assert_eq!(
                graph,
                expected,
                "{} produced the wrong graph at ({row_offset}, {column_offset})",
                pattern_rule.metadata().name()
            );
        }
    }
}

/// Clone a graph and move every position by the given offsets.
fn shift(graph: &Graph, row_offset: usize, column_offset: usize) -> Graph {
    let target_height = graph.height() + row_offset;

    let mut shifted = graph.clone();
    shifted.remap_positions(|position| {
        Position::new(
            position.row() + row_offset,
            position.column() + column_offset,
        )
    });
    pad_height(&mut shifted, target_height);
    shifted
}

/// Grow the graph height without leaving any node behind.
fn pad_height(graph: &mut Graph, height: usize) {
    while graph.height() < height {
        let position = Position::new(graph.height(), 0);
        graph
            .add_node(GateType::X, position, None, None, None, None)
            .expect("The filler position should be free");
        graph.remove_node(position);
    }
}

#[test]
fn replacement_is_placed_relative_to_the_matched_anchor() {
    // The pattern is shifted one column to the right by two leading identities.
    // The replacement must follow the matched anchor, not the pattern's own column.
    let rule = control_reversal::cx_reversal();
    let mut graph = GraphBuilder::new(2)
        .push_id(0)
        .push_id(1)
        .push_h(0)
        .push_h(1)
        .push_cx(0, 1)
        .unwrap()
        .push_h(0)
        .push_h(1)
        .build();

    assert!(rule.apply(&mut graph));

    let expected = GraphBuilder::new(2)
        .push_id(0)
        .push_id(1)
        .push_cx(1, 0)
        .unwrap()
        .build();

    assert_eq!(graph, expected);
}

#[test]
fn replacement_is_placed_relative_to_the_matched_rows() {
    // The pattern sits on qubits 1 and 2, with an unrelated identity on qubit 0.
    let rule = control_reversal::cx_reversal();
    let mut graph = GraphBuilder::new(3)
        .push_id(0)
        .push_id(1)
        .push_id(2)
        .push_h(1)
        .push_h(2)
        .push_cx(1, 2)
        .unwrap()
        .push_h(1)
        .push_h(2)
        .build();

    assert!(rule.apply(&mut graph));

    let expected = GraphBuilder::new(3)
        .push_id(0)
        .push_id(1)
        .push_id(2)
        .push_cx(2, 1)
        .unwrap()
        .build();

    assert_eq!(graph, expected);
}

#[test]
fn deletion_rule_removes_a_shifted_match() {
    // A leading X shifts the `H H` pattern to the second and third columns.
    let rule = redundancy::double_hadamard();
    let mut graph = GraphBuilder::new(1).push_x(0).push_h(0).push_h(0).build();

    assert!(rule.apply(&mut graph));

    let nodes: Vec<_> = graph
        .iter_nodes_ordered_by_column()
        .map(|node| (node.position(), node.r#type()))
        .collect();

    assert_eq!(nodes, vec![(Position::new(0, 0), GateType::X)]);
}

#[test]
fn deletion_reconnects_neighbors_across_gaps() {
    // Hand-built graph with empty columns around the matched `H H` pattern.
    let rule = redundancy::double_hadamard();

    let mut graph = Graph::default();
    add_connected_node(&mut graph, GateType::X, Position::new(0, 0));
    add_connected_node(&mut graph, GateType::H, Position::new(0, 2));
    add_connected_node(&mut graph, GateType::H, Position::new(0, 3));
    add_connected_node(&mut graph, GateType::X, Position::new(0, 5));

    assert!(rule.apply(&mut graph));

    let mut expected = Graph::default();
    add_connected_node(&mut expected, GateType::X, Position::new(0, 0));
    add_connected_node(&mut expected, GateType::X, Position::new(0, 5));

    assert_eq!(graph, expected);
}
