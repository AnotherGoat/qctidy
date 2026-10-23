#[cfg(feature = "codegen-openqasm3")]
mod openqasm3;
#[cfg(test)]
#[cfg(feature = "codegen-openqasm3")]
mod openqasm3_tests;
#[cfg(feature = "codegen-qiskit")]
mod qiskit;
#[cfg(test)]
#[cfg(feature = "codegen-qiskit")]
mod qiskit_tests;

use qctidy::{GateOperation, Graph};
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
                    Ok(qiskit::generate(graph, circuit_name.unwrap_or("circuit")))
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
                    Ok(openqasm3::generate(graph))
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

    fn render_operation(
        &self,
        operation: &GateOperation,
        target: CodeGenerationTarget,
        circuit_name: Option<&str>,
    ) -> Result<String, CodeGenerationError> {
        use CodeGenerationTarget::*;

        match target {
            Qiskit => {
                #[cfg(feature = "codegen-qiskit")]
                {
                    Ok(qiskit::render_operation(
                        operation,
                        circuit_name.unwrap_or("circuit"),
                    ))
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
                    let _: Option<&str> = circuit_name;
                    Ok(openqasm3::render_operation(operation))
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
