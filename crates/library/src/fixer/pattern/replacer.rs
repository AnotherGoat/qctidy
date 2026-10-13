use std::collections::HashSet;

use crate::{Circuit, GateOperation, Graph, NodeView, PatternMatch, PatternRule, Position};

pub(crate) fn apply_match(graph: &mut Graph, rule: &PatternRule, pattern_match: &PatternMatch) {
    remove_lhs_nodes(graph, pattern_match);
    insert_rhs_nodes(graph, rule, pattern_match);
    reconnect_rhs_edges(graph, rule, pattern_match);
}

/// The operations that replace the matched pattern, in source order.
pub(crate) fn replacement_operations(
    rule: &PatternRule,
    pattern_match: &PatternMatch,
) -> Vec<GateOperation> {
    // Map the right-hand side onto the graph rows and read it back as operations.
    let mut mapped_rhs = rule.rhs().clone();
    mapped_rhs.remap_positions(|position| {
        Position::new(
            pattern_match
                .mapping()
                .graph_row(position.row())
                .expect("RHS rows should be mapped"),
            position.column(),
        )
    });

    Circuit::from(&mapped_rhs).operations().clone()
}

fn remove_lhs_nodes(graph: &mut Graph, pattern_match: &PatternMatch) {
    for &position in pattern_match.covered_positions() {
        graph.remove_node(position);
    }
}

fn insert_rhs_nodes(graph: &mut Graph, rule: &PatternRule, pattern_match: &PatternMatch) {
    for rhs_node in rule.rhs().iter_nodes() {
        let Some(graph_position) =
            map_rhs_position_to_graph(rule, pattern_match, rhs_node.position())
        else {
            continue;
        };

        graph
            .add_node(
                rhs_node.r#type(),
                graph_position,
                rhs_node.theta(),
                rhs_node.phi(),
                rhs_node.lambda(),
                rhs_node.bit(),
            )
            .expect("LHS nodes should already be removed");
    }
}

fn reconnect_rhs_edges(graph: &mut Graph, rule: &PatternRule, pattern_match: &PatternMatch) {
    let inserted_positions = collect_inserted_rhs_positions(rule, pattern_match);

    for rhs_node in rule.rhs().iter_nodes() {
        if map_rhs_position_to_graph(rule, pattern_match, rhs_node.position()).is_none() {
            continue;
        }

        reconnect_rhs_node_edges(graph, rule, rhs_node, &inserted_positions, pattern_match);
    }
}

fn reconnect_rhs_node_edges(
    graph: &mut Graph,
    rule: &PatternRule,
    rhs_node: NodeView,
    inserted_positions: &HashSet<Position>,
    pattern_match: &PatternMatch,
) {
    for edge in rule.rhs().iter_edges_from_unique(rhs_node.position()) {
        let Some(mapped_start) =
            map_rhs_position_to_graph(rule, pattern_match, edge.start().position())
        else {
            continue;
        };

        let Some(mapped_end) =
            map_rhs_position_to_graph(rule, pattern_match, edge.end().position())
        else {
            continue;
        };

        if !inserted_positions.contains(&mapped_start) || !inserted_positions.contains(&mapped_end)
        {
            continue;
        }

        graph
            .add_edge(edge.r#type(), mapped_start, mapped_end)
            .expect("RHS edge endpoints should exist");
    }
}

fn collect_inserted_rhs_positions(
    rule: &PatternRule,
    pattern_match: &PatternMatch,
) -> HashSet<Position> {
    rule.rhs()
        .iter_nodes()
        .filter_map(|rhs_node| map_rhs_position_to_graph(rule, pattern_match, rhs_node.position()))
        .collect()
}

fn map_rhs_position_to_graph(
    rule: &PatternRule,
    pattern_match: &PatternMatch,
    rhs_position: Position,
) -> Option<Position> {
    let mapping = pattern_match.mapping();
    let graph_row = mapping.graph_row(rhs_position.row())?;

    // Offset between the anchor's position in the graph and in the pattern.
    let column_offset = pattern_match
        .anchor_position()
        .column()
        .checked_sub(rule.anchor().position().column())?;

    let graph_column = column_offset.checked_add(rhs_position.column())?;

    Some(Position::new(graph_row, graph_column))
}
