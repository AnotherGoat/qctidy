use itertools::Itertools;

use crate::{
    Graph, PatternMatch, PatternRule, Position,
    simplifier::pattern::{
        cache::GraphCache, occupancy::OccupancyMap, pattern_match::QubitMapping,
    },
};

pub(crate) fn find_matches(graph: &Graph, rule: &PatternRule) -> Vec<PatternMatch> {
    let mut matches = Vec::new();
    let mut occupancy = OccupancyMap::new(graph.height(), graph.width());
    let anchor = rule.anchor();

    for graph_node in graph.iter_nodes_ordered_by_column() {
        if graph_node.r#type() != anchor.gate_type() {
            continue;
        }

        let anchor_position = graph_node.position();

        for mapping in generate_mappings(graph, rule, anchor_position) {
            let Some(pattern_match) = match_at_with_mapping(graph, rule, &mapping, anchor_position)
            else {
                continue;
            };

            if !occupancy.can_occupy(pattern_match.covered_positions()) {
                continue;
            }

            occupancy.occupy_all(pattern_match.covered_positions());

            matches.push(pattern_match);

            break;
        }
    }

    matches
}

fn generate_mappings(
    graph: &Graph,
    rule: &PatternRule,
    anchor_position: Position,
) -> impl Iterator<Item = QubitMapping> {
    let anchor = rule.anchor();

    let anchor_pattern_row = anchor.row();
    let anchor_graph_row = anchor_position.row();

    let pattern_rows: Vec<_> = (0..rule.height())
        .filter(|&row| row != anchor_pattern_row)
        .collect();

    let graph_rows: Vec<_> = (0..graph.height())
        .filter(|&row| row != anchor_graph_row)
        .collect();

    let graph_height = graph.height();
    let pattern_height = rule.height();

    graph_rows
        .into_iter()
        .permutations(pattern_rows.len())
        .filter_map(move |permutation| {
            let mut mapping = QubitMapping::new(pattern_height, graph_height);

            if !mapping.add_mapping(anchor_pattern_row, anchor_graph_row) {
                return None;
            }

            for (&pattern_row, &graph_row) in pattern_rows.iter().zip(permutation.iter()) {
                if !mapping.add_mapping(pattern_row, graph_row) {
                    return None;
                }
            }

            let graph_cache = GraphCache::from_graph(graph);

            if !mapping_satisfies_row_cache(&graph_cache, rule, &mapping) {
                return None;
            }

            Some(mapping)
        })
}

fn mapping_satisfies_row_cache(
    graph_cache: &GraphCache,
    rule: &PatternRule,
    mapping: &QubitMapping,
) -> bool {
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

fn match_at_with_mapping(
    graph: &Graph,
    rule: &PatternRule,
    mapping: &QubitMapping,
    anchor_graph_position: Position,
) -> Option<PatternMatch> {
    let anchor = rule.anchor();

    let mut covered_positions = Vec::new();

    for pattern_node in rule.lhs().iter_nodes() {
        let pattern_position = pattern_node.position();

        let graph_row = mapping.graph_row(pattern_position.row())?;

        let graph_column =
            anchor_graph_position.column() + pattern_position.column() - anchor.column();

        let graph_position = Position::new(graph_row, graph_column);
        let graph_node = graph.get_node(graph_position)?;

        if graph_node.r#type() != pattern_node.r#type() {
            return None;
        }

        covered_positions.push(graph_position);
    }

    Some(PatternMatch::new(
        rule.metadata().id(),
        mapping.clone(),
        covered_positions,
    ))
}
