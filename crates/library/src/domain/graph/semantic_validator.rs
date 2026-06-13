use std::collections::{HashMap, HashSet};

use crate::{
    Graph, Position,
    domain::graph::{
        GraphError,
        schema::{EdgeSchema, GateSchema, NodeSchema},
    },
};

#[must_use]
struct ValidationContext<'a> {
    graph: &'a Graph,
    component: &'a [Position],
    schema: &'a GateSchema,
    count: usize,
    index_map: HashMap<Position, usize>,
    expected_edges: HashMap<EdgeSchema, usize>,
    actual_edges: HashMap<EdgeSchema, usize>,
}

/// Validate the semantic relationships in a `Graph`.
///
/// This only is part of the validation process. To fully validate a `Graph`, use `Graph::validate` instead.
pub(super) fn validate(graph: &Graph) -> Result<(), GraphError> {
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
    let gate_type = graph
        .get_node(component[0])
        .expect("Component should have at least one node")
        .r#type();

    if component.iter().any(|&position| {
        graph
            .get_node(position)
            .expect("Position in component should have a node")
            .r#type()
            != gate_type
    }) {
        return Err(GraphError::InvalidGateStructure);
    }

    let count = component.len();
    let schema = gate_type.schema();

    if count != schema.nodes().len() || has_invalid_external_edges(graph, component) {
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

fn expected_edges(schema: &GateSchema) -> HashMap<EdgeSchema, usize> {
    let mut map = HashMap::new();

    for edge in *schema.edges() {
        *map.entry(*edge).or_insert(0) += 1;
    }

    map
}

fn collect_edges(
    graph: &Graph,
    index_map: &HashMap<Position, usize>,
) -> HashMap<EdgeSchema, usize> {
    let mut edges = HashMap::new();

    for (&start, &from_index) in index_map {
        for edge in graph.iter_semantic_edges_out_from(start) {
            let end = edge.end().position();

            if let Some(&to_index) = index_map.get(&end) {
                *edges
                    .entry(EdgeSchema::new(edge.r#type(), from_index, to_index))
                    .or_insert(0) += 1;
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

        let node = context
            .graph
            .get_node(context.component[i])
            .expect("Component should have a node");
        let node_schema = &context.schema.nodes()[j];

        if node_schema.has_angle() != node.angle().is_some()
            || node_schema.has_bit() != node.bit().is_some()
        {
            continue;
        }

        if matches!(node_schema, NodeSchema::TripleAngle)
            && (node.angle2().is_none() || node.angle3().is_none())
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

            if let Some(&j) = context.index_map.get(&end)
                && j <= up_to
            {
                let key = EdgeSchema::new(edge.r#type(), mapping[k], mapping[j]);

                let count = partial.entry(key).or_insert(0);
                *count += 1;

                match context.expected_edges.get(&key) {
                    Some(&expected) if *count <= expected => {}
                    _ => return false,
                }
            }
        }
    }

    true
}

fn matches_edges(context: &ValidationContext, mapping: &[usize]) -> bool {
    let mut mapped = HashMap::new();

    for (edge, count) in &context.actual_edges {
        *mapped
            .entry(EdgeSchema::new(
                edge.r#type(),
                mapping[edge.from_index()],
                mapping[edge.to_index()],
            ))
            .or_insert(0) += count;
    }

    mapped == context.expected_edges
}
