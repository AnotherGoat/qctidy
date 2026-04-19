use qsimplify::{
    dto::{gate_operation::GateOperation, mapper},
    simplifier,
};

pub fn display_graph(operations: &[GateOperation]) -> String {
    let graph = mapper::operations_to_graph(operations);
    graph.to_string()
}

pub fn display_grid(operations: &[GateOperation]) -> String {
    let graph = mapper::operations_to_graph(operations);
    graph.display_grid()
}

pub fn simplify(operations: &[GateOperation], iterations: u32) -> Vec<GateOperation> {
    let graph = mapper::operations_to_graph(operations);
    let simplified = simplifier::simplify(graph, iterations);
    mapper::graph_to_operations(simplified)
}
