mod openqasm3;
mod qiskit;

#[cfg(test)]
mod openqasm3_tests;
#[cfg(test)]
mod qiskit_tests;

use qsimplify::Graph;
use qsimplify_ports::{CodeGenerationTarget, CodegenPort};

#[derive(Debug, Copy, Clone)]
pub struct CodegenAdapter;

impl CodegenPort for CodegenAdapter {
    fn generate(
        &self,
        graph: &Graph,
        target: CodeGenerationTarget,
        circuit_name: Option<&str>,
    ) -> String {
        use CodeGenerationTarget::*;

        match target {
            Qiskit => qiskit::generate(graph, circuit_name.unwrap_or("circuit")),
            OpenQasm3 => openqasm3::generate(graph),
        }
    }
}
