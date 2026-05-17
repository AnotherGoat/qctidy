use std::sync::Arc;

use getset::{CloneGetters, CopyGetters, Getters};
use inew::New;
use qsimplify::{Circuit, mapper};
use qsimplify_ports::{CodeGenerationError, CodeGenerationTarget, CodegenPort};

#[derive(Debug, Clone, Getters, CloneGetters, CopyGetters, New)]
#[new(pub, const)]
#[must_use]
pub struct CodeGenerationRequest {
    #[get_clone = "pub"]
    circuit: Arc<Circuit>,
    #[get_copy = "pub"]
    target: CodeGenerationTarget,
    #[get = "pub"]
    circuit_name: Option<String>,
}

#[derive(Debug, Clone, Getters, New)]
#[new(pub, const)]
#[must_use]
pub struct CodeGenerationResponse {
    #[get = "pub"]
    code: String,
}

pub fn generate_code<C: CodegenPort>(
    request: &CodeGenerationRequest,
    codegen: &C,
) -> Result<CodeGenerationResponse, CodeGenerationError> {
    let graph = mapper::circuit_to_graph(request.circuit().as_ref());

    let result = codegen.generate(&graph, request.target(), request.circuit_name().as_deref());
    Ok(CodeGenerationResponse::new(result))
}
