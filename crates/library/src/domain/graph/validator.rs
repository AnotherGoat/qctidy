use std::collections::{HashMap, HashSet};

use crate::domain::{
    EdgeType, Graph, Position,
    graph::{EdgeData, GraphError, schema::GateSchema},
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

pub(crate) fn validate_graph_structure(graph: &Graph) -> Result<(), GraphError> {
    let components = find_components(graph);
    let mut covered = HashSet::new();

    for component in &components {
        validate_component(graph, component)?;

        for &pos in component {
            if !covered.insert(pos) {
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

    for &start in graph.nodes.keys() {
        if visited.contains(&start) {
            continue;
        }

        let mut stack = vec![start];
        let mut component = Vec::new();

        while let Some(position) = stack.pop() {
            if !visited.insert(position) {
                continue;
            }

            component.push(position);

            for edge in graph.edges_out.get(&position).into_iter().flatten() {
                match edge.edge_type {
                    EdgeType::Targets
                    | EdgeType::ControlledBy
                    | EdgeType::WorksWith
                    | EdgeType::SwapsWith => {
                        stack.push(edge.other);
                    }
                    _ => {}
                }
            }
        }

        components.push(component);
    }

    components
}

fn validate_component(graph: &Graph, component: &[Position]) -> Result<(), GraphError> {
    let gate = graph.nodes[&component[0]].gate;

    if component
        .iter()
        .any(|position| graph.nodes[position].gate != gate)
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

    for position in component {
        if let Some(edges) = graph.edges_out.get(&position) {
            if contains_invalid_edge(&set, edges) {
                return true;
            }
        }

        if let Some(edges) = graph.edges_in.get(&position) {
            if contains_invalid_edge(&set, edges) {
                return true;
            }
        }
    }

    false
}

fn contains_invalid_edge(set: &HashSet<Position>, edges: &Vec<EdgeData>) -> bool {
    for edge in edges {
        if edge.edge_type.is_semantic() && !set.contains(&edge.other) {
            return true;
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

    for (&from_position, &i) in index_map {
        if let Some(out) = graph.edges_out.get(&from_position) {
            for edge in out {
                if edge.edge_type.is_positional() {
                    continue;
                }

                if let Some(&j) = index_map.get(&edge.other) {
                    *edges.entry((edge.edge_type, i, j)).or_insert(0) += 1;
                }
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

        let node = &context.graph.nodes[&context.component[i]];
        let node_schema = &context.schema.nodes[j];

        if node_schema.has_angle() != node.angle.is_some()
            || node_schema.has_bit() != node.bit.is_some()
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
        let from_position = context.component[k];

        if let Some(out) = context.graph.edges_out.get(&from_position) {
            for edge in out {
                if edge.edge_type.is_positional() {
                    continue;
                }

                if let Some(&j) = context.index_map.get(&edge.other) {
                    if j <= up_to {
                        let key = (edge.edge_type, mapping[k], mapping[j]);

                        let count = partial.entry(key).or_insert(0);
                        *count += 1;

                        if let Some(&expected_count) = context.expected_edges.get(&key) {
                            if *count > expected_count {
                                return false;
                            }
                        } else {
                            return false;
                        }
                    }
                }
            }
        }
    }

    true
}

fn matches_edges(context: &ValidationContext, mapping: &[usize]) -> bool {
    let mut mapped = HashMap::new();

    for ((ty, from, to), count) in &context.actual_edges {
        *mapped
            .entry((*ty, mapping[*from], mapping[*to]))
            .or_insert(0) += count;
    }

    mapped == context.expected_edges
}
