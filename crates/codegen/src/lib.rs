#[cfg(feature = "codegen-openqasm3")]
mod openqasm3;
#[cfg(feature = "codegen-qiskit")]
mod qiskit;

#[cfg(test)]
#[cfg(feature = "codegen-openqasm3")]
mod openqasm3_tests;
#[cfg(test)]
#[cfg(feature = "codegen-qiskit")]
mod qiskit_tests;

use qctidy::Graph;
use qctidy_ports::{CodeGenerationError, CodeGenerationTarget, CodegenPort};

#[derive(Debug, Copy, Clone)]
pub struct CodegenAdapter;

impl CodegenPort for CodegenAdapter {
    fn generate(
        &self,
        graph: &Graph,
        target: CodeGenerationTarget,
        circuit_name: Option<&str>,
    ) -> Result<String, CodeGenerationError> {
        use CodeGenerationTarget::*;

        match target {
            Qiskit => {
                #[cfg(feature = "codegen-qiskit")]
                {
                    qiskit::generate(graph, circuit_name.unwrap_or("circuit"))
                }
                #[cfg(not(feature = "codegen-qiskit"))]
                {
                    Err(CodeGenerationError::MissingFeature {
                        feature: "codegen-qiskit",
                        target,
                    })
                }
            }
            OpenQasm3 => {
                #[cfg(feature = "codegen-openqasm3")]
                {
                    openqasm3::generate(graph)
                }
                #[cfg(not(feature = "codegen-openqasm3"))]
                {
                    Err(CodeGenerationError::MissingFeature {
                        feature: "codegen-openqasm3",
                        target,
                    })
                }
            }
        }
    }
}
