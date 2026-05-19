use qsimplify::Graph;
use thiserror::Error;

pub trait CodegenPort {
    fn generate(
        &self,
        graph: &Graph,
        target: CodeGenerationTarget,
        circuit_name: Option<&str>,
    ) -> String;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CodeGenerationTarget {
    Qiskit,
    OpenQasm3,
}

#[derive(Debug, Clone, Copy, Error)]
pub enum CodeGenerationError {}
