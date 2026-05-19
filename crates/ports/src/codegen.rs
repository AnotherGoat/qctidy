use qsimplify::Graph;
use thiserror::Error;

pub trait CodegenPort {
    fn generate(
        &self,
        graph: &Graph,
        target: CodeGenerationTarget,
        circuit_name: Option<&str>,
    ) -> Result<String, CodeGenerationError>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, strum_macros::Display)]
pub enum CodeGenerationTarget {
    Qiskit,
    OpenQasm3,
}

impl CodeGenerationTarget {
    pub const fn is_available(self) -> bool {
        use CodeGenerationTarget::*;

        match self {
            Qiskit => cfg!(feature = "codegen-qiskit"),
            OpenQasm3 => cfg!(feature = "codegen-openqasm3"),
        }
    }
}

#[derive(Debug, Clone, Copy, Error)]
pub enum CodeGenerationError {
    #[error("Missing feature '{feature}' required for target '{target}'")]
    MissingFeature {
        feature: &'static str,
        target: CodeGenerationTarget,
    },
}
