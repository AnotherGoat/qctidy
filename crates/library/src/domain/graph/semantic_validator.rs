use std::collections::{HashMap, HashSet};

use crate::domain::{
    EdgeType, Graph, Position,
    graph::{GraphError, schema::GateSchema},
};

struct ValidationContext<'a> {
    graph: &'a Graph,
    component: &'a [Position],
    schema: &'a GateSchema,
    count: usize,
    index_map: HashMap<Position, usize>,
    expected_edges: HashMap<(EdgeType, usize, usize), usize>,
    actual_edges: HashMap<(EdgeType, usize, usize), usize>,
}

/// Validate the semantic relationships in a `Graph`.
///
/// This only is part of the validation process. To fully validate a `Graph`, use `Graph::validate` instead.
pub(crate) fn validate(graph: &Graph) -> Result<(), GraphError> {
    let components = find_components(graph);
    let mut covered = HashSet::new();

    for component in &components {
        validate_component(graph, component)?;

        for &position in component {
            if !covered.insert(position) {
                return Err(GraphError::InvalidGateStructure);
            }
        }
    }

    if covered.len() != graph.nodes.len() {
        return Err(GraphError::InvalidGateStructure);
    }

    Ok(())
}

fn find_components(graph: &Graph) -> Vec<Vec<Position>> {
    let mut visited = HashSet::new();
    let mut components = Vec::new();

    for position in graph.iter_positions_ordered_by_row() {
        if visited.contains(&position) {
            continue;
        }

        let mut stack = vec![position];
        let mut component = Vec::new();

        while let Some(visiting) = stack.pop() {
            if !visited.insert(visiting) {
                continue;
            }

            component.push(visiting);

            for neighbor in graph.iter_semantic_neighbors_from(visiting) {
                stack.push(neighbor);
            }
        }

        components.push(component);
    }

    components
}

fn validate_component(graph: &Graph, component: &[Position]) -> Result<(), GraphError> {
    let gate = graph.get_node(component[0]).unwrap().r#type();

    if component
        .iter()
        .any(|&p| graph.get_node(p).unwrap().r#type() != gate)
    {
        return Err(GraphError::InvalidGateStructure);
    }

    let count = component.len();
    let schema = gate.schema();

    if count != schema.nodes.len() || has_invalid_external_edges(graph, component) {
        return Err(GraphError::InvalidGateStructure);
    }

    let index_map = build_index_map(component);
    let expected_edges = expected_edges(schema);
    let actual_edges = collect_edges(graph, &index_map);

    let context = ValidationContext {
        graph,
        component,
        schema,
        count,
        index_map,
        expected_edges,
        actual_edges,
    };

    if try_permutations(&context) {
        return Ok(());
    }

    Err(GraphError::InvalidGateStructure)
}

fn has_invalid_external_edges(graph: &Graph, component: &[Position]) -> bool {
    let set: HashSet<_> = component.iter().copied().collect();

    for &position in component {
        for edge in graph.iter_semantic_edges_out_from(position) {
            let target = edge.end().position();
            if !set.contains(&target) {
                return true;
            }
        }
    }

    false
}

fn expected_edges(schema: &GateSchema) -> HashMap<(EdgeType, usize, usize), usize> {
    let mut map = HashMap::new();

    for edge in schema.edges {
        *map.entry((edge.edge_type, edge.from, edge.to)).or_insert(0) += 1;
    }

    map
}

fn collect_edges(
    graph: &Graph,
    index_map: &HashMap<Position, usize>,
) -> HashMap<(EdgeType, usize, usize), usize> {
    let mut edges = HashMap::new();

    for (&start, &i) in index_map {
        for edge in graph.iter_semantic_edges_out_from(start) {
            let end = edge.end().position();

            if let Some(&j) = index_map.get(&end) {
                *edges.entry((edge.r#type(), i, j)).or_insert(0) += 1;
            }
        }
    }

    edges
}

fn build_index_map(component: &[Position]) -> HashMap<Position, usize> {
    component
        .iter()
        .enumerate()
        .map(|(index, &position)| (position, index))
        .collect()
}

fn try_permutations(context: &ValidationContext) -> bool {
    let mut used = vec![false; context.count];
    let mut mapping = vec![0; context.count];

    backtrack(context, 0, &mut used, &mut mapping)
}

fn backtrack(
    context: &ValidationContext,
    i: usize,
    used: &mut [bool],
    mapping: &mut [usize],
) -> bool {
    if i == context.count {
        return matches_edges(context, mapping);
    }

    for j in 0..context.count {
        if used[j] {
            continue;
        }

        let node = context.graph.get_node(context.component[i]).unwrap();
        let node_schema = &context.schema.nodes[j];

        if node_schema.has_angle() != node.angle().is_some()
            || node_schema.has_bit() != node.bit().is_some()
        {
            continue;
        }

        used[j] = true;
        mapping[i] = j;

        if !partial_edges_match(context, mapping, i) {
            used[j] = false;
            continue;
        }

        if backtrack(context, i + 1, used, mapping) {
            return true;
        }

        used[j] = false;
    }

    false
}

fn partial_edges_match(context: &ValidationContext, mapping: &[usize], up_to: usize) -> bool {
    let mut partial = HashMap::new();

    for k in 0..=up_to {
        let start = context.component[k];

        for edge in context.graph.iter_semantic_edges_out_from(start) {
            let end = edge.end().position();

            if let Some(&j) = context.index_map.get(&end) {
                if j <= up_to {
                    let key = (edge.r#type(), mapping[k], mapping[j]);

                    let count = partial.entry(key).or_insert(0);
                    *count += 1;

                    match context.expected_edges.get(&key) {
                        Some(&expected) if *count <= expected => {}
                        _ => return false,
                    }
                }
            }
        }
    }

    true
}

fn matches_edges(context: &ValidationContext, mapping: &[usize]) -> bool {
    let mut mapped = HashMap::new();

    for ((r#type, from, to), count) in &context.actual_edges {
        *mapped
            .entry((*r#type, mapping[*from], mapping[*to]))
            .or_insert(0) += count;
    }

    mapped == context.expected_edges
}
