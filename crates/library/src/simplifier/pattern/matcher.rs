use std::collections::HashMap;

use itertools::Itertools;

use crate::{
    EdgeType, EdgeView, Graph, NodeView, PatternMatch, PatternRule, Position,
    simplifier::pattern::{
        cache::GraphCache, occupancy::OccupancyMap, pattern_match::QubitMapping,
    },
};

pub(crate) fn find_matches(graph: &Graph, rule: &PatternRule) -> Vec<PatternMatch> {
    let mut matches = Vec::new();
    let mut occupancy = OccupancyMap::new(graph.height(), graph.width());

    for node in graph.iter_nodes_ordered_by_column() {
        if !is_anchor_candidate(rule, node) {
            continue;
        }

        let Some(r#match) = find_match_at_anchor(graph, rule, node.position(), &occupancy) else {
            continue;
        };

        occupancy.occupy_all(r#match.covered_positions());
        matches.push(r#match);
    }

    matches
}

fn is_anchor_candidate(rule: &PatternRule, node: NodeView) -> bool {
    node.r#type() == rule.anchor().gate_type()
}

fn find_match_at_anchor(
    graph: &Graph,
    rule: &PatternRule,
    anchor_position: Position,
    occupancy: &OccupancyMap,
) -> Option<PatternMatch> {
    for mapping in generate_mappings(graph, rule, anchor_position) {
        let Some(pattern_match) = build_pattern_match(graph, rule, &mapping, anchor_position)
        else {
            continue;
        };

        if !occupancy.can_occupy(pattern_match.covered_positions()) {
            continue;
        }

        return Some(pattern_match);
    }

    None
}

fn generate_mappings(
    graph: &Graph,
    rule: &PatternRule,
    anchor_position: Position,
) -> impl Iterator<Item = QubitMapping> {
    let anchor_pattern_row = rule.anchor().position().row();
    let anchor_graph_row = anchor_position.row();

    let pattern_rows = collect_non_anchor_pattern_rows(rule);
    let graph_rows = collect_non_anchor_graph_rows(graph, anchor_graph_row);

    graph_rows
        .into_iter()
        .permutations(pattern_rows.len())
        .filter_map(move |permutation| {
            build_mapping(
                graph,
                rule,
                anchor_pattern_row,
                anchor_graph_row,
                &pattern_rows,
                &permutation,
            )
        })
}

fn collect_non_anchor_pattern_rows(rule: &PatternRule) -> Vec<usize> {
    let anchor_row = rule.anchor().position().row();

    (0..rule.height())
        .filter(|&row| row != anchor_row)
        .collect()
}

fn collect_non_anchor_graph_rows(graph: &Graph, anchor_graph_row: usize) -> Vec<usize> {
    (0..graph.height())
        .filter(|&row| row != anchor_graph_row)
        .collect()
}

fn build_mapping(
    graph: &Graph,
    rule: &PatternRule,
    anchor_pattern_row: usize,
    anchor_graph_row: usize,
    pattern_rows: &[usize],
    permutation: &[usize],
) -> Option<QubitMapping> {
    let mut mapping = QubitMapping::new(rule.height(), graph.height());

    mapping
        .add_mapping(anchor_pattern_row, anchor_graph_row)
        .then_some(())?;

    add_permuted_row_mappings(&mut mapping, pattern_rows, permutation)?;

    mapping_satisfies_cache(graph, rule, &mapping).then_some(mapping)
}

fn add_permuted_row_mappings(
    mapping: &mut QubitMapping,
    pattern_rows: &[usize],
    permutation: &[usize],
) -> Option<()> {
    for (&pattern_row, &graph_row) in pattern_rows.iter().zip(permutation.iter()) {
        mapping.add_mapping(pattern_row, graph_row).then_some(())?;
    }

    Some(())
}

fn mapping_satisfies_cache(graph: &Graph, rule: &PatternRule, mapping: &QubitMapping) -> bool {
    let graph_cache = GraphCache::from_graph(graph);

    for pattern_row in 0..rule.height() {
        let Some(graph_row) = mapping.graph_row(pattern_row) else {
            return false;
        };

        let Some(rule_row_cache) = rule.lhs_cache().row(pattern_row) else {
            return false;
        };

        let Some(graph_row_cache) = graph_cache.row(graph_row) else {
            return false;
        };

        if !graph_row_cache.is_superset_of(rule_row_cache) {
            return false;
        }
    }

    true
}

fn build_pattern_match(
    graph: &Graph,
    rule: &PatternRule,
    mapping: &QubitMapping,
    anchor_graph_position: Position,
) -> Option<PatternMatch> {
    let matched_nodes = collect_matched_nodes(graph, rule, mapping, anchor_graph_position)?;

    relationships_match(rule.lhs(), graph, &matched_nodes).then_some(())?;

    let covered_positions = collect_covered_positions(&matched_nodes);

    Some(PatternMatch::new(
        rule.metadata().id(),
        mapping.clone(),
        covered_positions,
    ))
}

fn collect_matched_nodes(
    graph: &Graph,
    rule: &PatternRule,
    mapping: &QubitMapping,
    anchor_graph_position: Position,
) -> Option<Vec<(NodeView, NodeView)>> {
    rule.lhs()
        .iter_nodes()
        .map(|pattern_node| {
            match_pattern_node(graph, rule, mapping, anchor_graph_position, pattern_node)
        })
        .collect()
}

fn match_pattern_node(
    graph: &Graph,
    rule: &PatternRule,
    mapping: &QubitMapping,
    anchor_graph_position: Position,
    pattern_node: NodeView,
) -> Option<(NodeView, NodeView)> {
    let graph_position = map_pattern_position_to_graph(
        rule,
        mapping,
        anchor_graph_position,
        pattern_node.position(),
    )?;

    let graph_node = graph.get_node(graph_position)?;

    nodes_match(pattern_node, graph_node).then_some((pattern_node, graph_node))
}

fn map_pattern_position_to_graph(
    rule: &PatternRule,
    mapping: &QubitMapping,
    anchor_graph_position: Position,
    pattern_position: Position,
) -> Option<Position> {
    let graph_row = mapping.graph_row(pattern_position.row())?;

    let graph_column =
        map_pattern_column_to_graph(rule, anchor_graph_position, pattern_position.column());

    Some(Position::new(graph_row, graph_column))
}

fn map_pattern_column_to_graph(
    rule: &PatternRule,
    anchor_graph_position: Position,
    pattern_column: usize,
) -> usize {
    anchor_graph_position.column() + pattern_column - rule.anchor().position().column()
}

fn nodes_match(pattern_node: NodeView, graph_node: NodeView) -> bool {
    pattern_node.r#type() == graph_node.r#type()
}

fn relationships_match(
    pattern_graph: &Graph,
    graph: &Graph,
    matched_nodes: &[(NodeView, NodeView)],
) -> bool {
    let mapping = build_position_mapping(matched_nodes);

    matched_nodes.iter().all(|(pattern_node, graph_node)| {
        node_relationships_match(pattern_graph, graph, pattern_node, graph_node, &mapping)
    })
}

fn node_relationships_match(
    pattern_graph: &Graph,
    graph: &Graph,
    pattern_node: &NodeView,
    graph_node: &NodeView,
    mapping: &HashMap<Position, Position>,
) -> bool {
    pattern_graph
        .iter_edges_from_unique(pattern_node.position())
        .all(|pattern_edge| mapped_edge_exists(graph, graph_node.position(), pattern_edge, mapping))
}

fn mapped_edge_exists(
    graph: &Graph,
    graph_node_position: Position,
    pattern_edge: EdgeView,
    mapping: &HashMap<Position, Position>,
) -> bool {
    let Some((expected_type, expected_start, expected_end)) =
        map_pattern_edge(pattern_edge, mapping)
    else {
        return false;
    };

    graph
        .iter_edges_from_unique(graph_node_position)
        .any(|graph_edge| {
            graph_edge.r#type() == expected_type
                && graph_edge.start().position() == expected_start
                && graph_edge.end().position() == expected_end
        })
}

fn map_pattern_edge(
    pattern_edge: EdgeView,
    mapping: &HashMap<Position, Position>,
) -> Option<(EdgeType, Position, Position)> {
    let start = mapping.get(&pattern_edge.start().position())?;

    let end = mapping.get(&pattern_edge.end().position())?;

    Some((pattern_edge.r#type(), *start, *end))
}

fn collect_covered_positions(matched_nodes: &[(NodeView, NodeView)]) -> Vec<Position> {
    matched_nodes
        .iter()
        .map(|(_, graph_node)| graph_node.position())
        .collect()
}

fn build_position_mapping(matched_nodes: &[(NodeView, NodeView)]) -> HashMap<Position, Position> {
    matched_nodes
        .iter()
        .map(|(pattern_node, graph_node)| (pattern_node.position(), graph_node.position()))
        .collect()
}
