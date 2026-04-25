use qsimplify::{dto::GateOperation, dto::mapper, simplifier};

#[must_use]
pub fn display_graph(operations: &[GateOperation]) -> String {
    let graph = mapper::operations_to_graph(operations);
    graph.to_string()
}

#[must_use]
pub fn display_grid(operations: &[GateOperation]) -> String {
    let graph = mapper::operations_to_graph(operations);
    graph.display_grid()
}

#[must_use]
pub fn simplify(operations: &[GateOperation], iterations: u32) -> Vec<GateOperation> {
    let graph = mapper::operations_to_graph(operations);
    let simplified = simplifier::simplify(graph, iterations);
    mapper::graph_to_operations(&simplified)
}
