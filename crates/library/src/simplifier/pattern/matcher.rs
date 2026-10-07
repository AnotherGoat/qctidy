use std::collections::HashMap;

use crate::{
    EdgeType, EdgeView, Graph, NodeView, PatternMatch, PatternRule, Position,
    domain::math,
    simplifier::pattern::{
        cache::GraphCache, occupancy::OccupancyMap, pattern_match::QubitMapping,
    },
};

pub(crate) fn find_matches(graph: &Graph, rule: &PatternRule) -> Vec<PatternMatch> {
    let graph_cache = GraphCache::from_graph(graph);

    find_matches_with_cache(graph, rule, &graph_cache)
}

/// Find all matches reusing a cache of the graph, which is valid while the graph is unchanged.
pub(crate) fn find_matches_with_cache(
    graph: &Graph,
    rule: &PatternRule,
    graph_cache: &GraphCache,
) -> Vec<PatternMatch> {
    let mut matches = Vec::new();
    let mut occupancy = OccupancyMap::new(graph.height(), graph.width());

    for node in graph.iter_nodes_ordered_by_column() {
        if !is_anchor_candidate(rule, node) || !anchor_fits(rule, graph, node.position()) {
            continue;
        }

        let Some(found_match) =
            find_match_at_anchor(graph, rule, graph_cache, node.position(), &occupancy)
        else {
            continue;
        };

        occupancy.occupy_all(found_match.covered_positions());
        matches.push(found_match);
    }

    matches
}

fn is_anchor_candidate(rule: &PatternRule, node: NodeView) -> bool {
    node.r#type() == rule.anchor().gate_type()
}

fn anchor_fits(rule: &PatternRule, graph: &Graph, anchor_position: Position) -> bool {
    // Whether the pattern can fit in the graph with the anchor at the given position.
    let anchor_pattern_column = rule.anchor().position().column();
    let last_pattern_column = rule.width() - 1;

    anchor_position.column() >= anchor_pattern_column
        && anchor_position.column() + (last_pattern_column - anchor_pattern_column) < graph.width()
}

/// State shared by the recursive search for a mapping.
struct SearchContext<'a> {
    graph: &'a Graph,
    rule: &'a PatternRule,
    anchor_position: Position,
    occupancy: &'a OccupancyMap,
    /// Pattern rows that still need a graph row, most constrained first.
    pattern_rows: &'a [usize],
    /// Candidate graph rows for the pattern row at the same index of `pattern_rows`.
    candidate_rows: &'a [Vec<usize>],
}

fn find_match_at_anchor(
    graph: &Graph,
    rule: &PatternRule,
    graph_cache: &GraphCache,
    anchor_position: Position,
    occupancy: &OccupancyMap,
) -> Option<PatternMatch> {
    // Find the first match of the rule with its anchor at the given graph position.
    // The anchor row is fixed, so its pattern nodes must match before searching.
    if !anchor_row_matches(graph, rule, anchor_position) {
        return None;
    }

    let mut rows_to_assign: Vec<(usize, Vec<usize>)> = collect_non_anchor_pattern_rows(rule)
        .into_iter()
        .map(|pattern_row| {
            let candidates =
                candidate_graph_rows(graph, rule, graph_cache, anchor_position, pattern_row);

            (pattern_row, candidates)
        })
        .collect();

    // Assign the most constrained pattern rows first, so impossible branches fail early.
    rows_to_assign.sort_by_key(|(_, candidates)| candidates.len());

    let pattern_rows: Vec<usize> = rows_to_assign
        .iter()
        .map(|(pattern_row, _)| *pattern_row)
        .collect();
    let candidate_rows: Vec<Vec<usize>> = rows_to_assign
        .into_iter()
        .map(|(_, candidates)| candidates)
        .collect();

    let context = SearchContext {
        graph,
        rule,
        anchor_position,
        occupancy,
        pattern_rows: &pattern_rows,
        candidate_rows: &candidate_rows,
    };

    let mut mapping = QubitMapping::new(rule.height());
    mapping.add_mapping(rule.anchor().position().row(), anchor_position.row());

    let mut used_graph_rows = vec![false; graph.height()];
    used_graph_rows[anchor_position.row()] = true;

    search_mapping(&context, 0, &mut mapping, &mut used_graph_rows)
}

fn search_mapping(
    context: &SearchContext<'_>,
    depth: usize,
    mapping: &mut QubitMapping,
    used_graph_rows: &mut [bool],
) -> Option<PatternMatch> {
    // Assign a distinct graph row to every pattern row, trying the candidates in order.
    //
    // Already-assigned pattern edges are checked after each assignment to prune the search.
    if depth == context.pattern_rows.len() {
        return finish_match(context, mapping);
    }

    let pattern_row = context.pattern_rows[depth];

    for &graph_row in &context.candidate_rows[depth] {
        if used_graph_rows[graph_row] {
            continue;
        }

        mapping.add_mapping(pattern_row, graph_row);
        used_graph_rows[graph_row] = true;

        let found_match = if assigned_edges_match(context, mapping) {
            search_mapping(context, depth + 1, mapping, used_graph_rows)
        } else {
            None
        };

        used_graph_rows[graph_row] = false;
        mapping.remove_mapping(pattern_row);

        if found_match.is_some() {
            return found_match;
        }
    }

    None
}

fn finish_match(context: &SearchContext<'_>, mapping: &QubitMapping) -> Option<PatternMatch> {
    // Validate the complete mapping and build the match, if its positions are still free.
    let pattern_match = build_pattern_match(
        context.graph,
        context.rule,
        mapping,
        context.anchor_position,
    )?;

    context
        .occupancy
        .can_occupy(pattern_match.covered_positions())
        .then_some(pattern_match)
}

fn anchor_row_matches(graph: &Graph, rule: &PatternRule, anchor_position: Position) -> bool {
    // Check the pattern nodes that share the anchor's row, whose graph row is fixed.
    let anchor_pattern_row = rule.anchor().position().row();

    pattern_nodes_in_row(rule, anchor_pattern_row).all(|pattern_node| {
        pattern_node_matches_at(
            graph,
            rule,
            anchor_position,
            pattern_node,
            anchor_position.row(),
        )
    })
}

fn candidate_graph_rows(
    graph: &Graph,
    rule: &PatternRule,
    graph_cache: &GraphCache,
    anchor_position: Position,
    pattern_row: usize,
) -> Vec<usize> {
    // Graph rows that can host the given pattern row, excluding the anchor row.
    (0..graph.height())
        .filter(|&graph_row| graph_row != anchor_position.row())
        .filter(|&graph_row| {
            graph_row_matches_pattern_row(
                graph,
                rule,
                graph_cache,
                anchor_position,
                pattern_row,
                graph_row,
            )
        })
        .collect()
}

fn graph_row_matches_pattern_row(
    graph: &Graph,
    rule: &PatternRule,
    graph_cache: &GraphCache,
    anchor_position: Position,
    pattern_row: usize,
    graph_row: usize,
) -> bool {
    // Whether the graph row can host every pattern node of the given pattern row.
    // Fast filter: the row must contain every gate type of the pattern row.
    let Some(rule_row_cache) = rule.lhs_cache().row(pattern_row) else {
        return false;
    };
    let Some(graph_row_cache) = graph_cache.row(graph_row) else {
        return false;
    };
    if !graph_row_cache.is_superset_of(*rule_row_cache) {
        return false;
    }

    // Exact filter: every pattern node must match at its mapped position.
    pattern_nodes_in_row(rule, pattern_row).all(|pattern_node| {
        pattern_node_matches_at(graph, rule, anchor_position, pattern_node, graph_row)
    })
}

fn pattern_node_matches_at(
    graph: &Graph,
    rule: &PatternRule,
    anchor_position: Position,
    pattern_node: NodeView,
    graph_row: usize,
) -> bool {
    // Whether the pattern node matches the graph node at its mapped position.
    let Some(graph_column) =
        map_pattern_column_to_graph(rule, anchor_position, pattern_node.position().column())
    else {
        return false;
    };

    graph
        .get_node(Position::new(graph_row, graph_column))
        .is_some_and(|graph_node| nodes_match(pattern_node, graph_node))
}

fn pattern_nodes_in_row(
    rule: &PatternRule,
    pattern_row: usize,
) -> impl Iterator<Item = NodeView> + '_ {
    // The pattern nodes that belong to the given pattern row.
    rule.lhs()
        .iter_nodes()
        .filter(move |pattern_node| pattern_node.position().row() == pattern_row)
}

fn assigned_edges_match(context: &SearchContext<'_>, mapping: &QubitMapping) -> bool {
    // Check the pattern edges whose two endpoints are already assigned.
    context
        .rule
        .lhs()
        .iter_edges_unique()
        .all(|pattern_edge| assigned_edge_matches(context, mapping, pattern_edge))
}

fn assigned_edge_matches(
    context: &SearchContext<'_>,
    mapping: &QubitMapping,
    pattern_edge: EdgeView,
) -> bool {
    // Check a single pattern edge, if both of its endpoints are already assigned.
    let Some(start) = map_pattern_position_to_graph(
        context.rule,
        mapping,
        context.anchor_position,
        pattern_edge.start().position(),
    ) else {
        // The start is not assigned yet; the edge is checked once it is.
        return true;
    };

    let Some(end) = map_pattern_position_to_graph(
        context.rule,
        mapping,
        context.anchor_position,
        pattern_edge.end().position(),
    ) else {
        return true;
    };

    context.graph.iter_edges_from(start).any(|graph_edge| {
        graph_edge.r#type() == pattern_edge.r#type() && graph_edge.end().position() == end
    })
}

fn collect_non_anchor_pattern_rows(rule: &PatternRule) -> Vec<usize> {
    let anchor_row = rule.anchor().position().row();

    (0..rule.height())
        .filter(|&row| row != anchor_row)
        .collect()
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
        map_pattern_column_to_graph(rule, anchor_graph_position, pattern_position.column())?;

    Some(Position::new(graph_row, graph_column))
}

fn map_pattern_column_to_graph(
    rule: &PatternRule,
    anchor_graph_position: Position,
    pattern_column: usize,
) -> Option<usize> {
    let base = anchor_graph_position
        .column()
        .checked_sub(rule.anchor().position().column())?;

    base.checked_add(pattern_column)
}

fn nodes_match(pattern_node: NodeView, graph_node: NodeView) -> bool {
    pattern_node.r#type() == graph_node.r#type()
        && node_parameters_match(pattern_node.theta(), graph_node.theta())
        && node_parameters_match(pattern_node.phi(), graph_node.phi())
        && node_parameters_match(pattern_node.lambda(), graph_node.lambda())
        && pattern_node.bit() == graph_node.bit()
}

fn node_parameters_match(pattern_parameter: Option<f64>, graph_parameter: Option<f64>) -> bool {
    match (pattern_parameter, graph_parameter) {
        (Some(pattern_parameter), Some(graph_parameter))
            if math::are_floats_equal(pattern_parameter, 0.0) =>
        {
            math::is_float_close_to_zero(graph_parameter)
        }
        _ => math::are_option_floats_equal(pattern_parameter, graph_parameter),
    }
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
