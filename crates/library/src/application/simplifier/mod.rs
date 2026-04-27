/// Contains unitary matrix calculations for quantum graphs.
pub(crate) mod matrix_calculator;

#[cfg(test)]
mod matrix_calculator_tests;

use crate::Graph;

pub const fn simplify(graph: Graph, _iterations: u32) -> Graph {
    graph
}
