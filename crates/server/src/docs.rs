use utoipa::OpenApi as OpenApiMacro;
use utoipa::openapi::OpenApi;

use crate::schema;
use crate::use_cases::query::{features, health};

#[cfg(any(
    feature = "converter-cbor",
    feature = "converter-json",
    feature = "converter-msgpack",
    feature = "converter-xml",
))]
use crate::use_cases::command::{convert_circuit, display_circuit, simplify_circuit};

#[cfg(feature = "estimator")]
use crate::use_cases::command::estimate_circuit;

#[cfg(all(
    any(feature = "codegen-qiskit", feature = "codegen-openqasm3"),
    any(
        feature = "converter-cbor",
        feature = "converter-json",
        feature = "converter-msgpack",
        feature = "converter-xml",
    ),
))]
use crate::use_cases::command::generate_code;

#[cfg(all(
    feature = "presenter-graphviz",
    any(
        feature = "converter-cbor",
        feature = "converter-json",
        feature = "converter-msgpack",
        feature = "converter-xml",
    ),
))]
use crate::use_cases::command::present_circuit;

#[derive(OpenApiMacro)]
#[openapi(
    info(
        title = "QSimplify API",
        description = "REST API for QSimplify — quantum circuit simplification, analysis, and visualization",
    ),
    paths(health::handler, features::handler),
    components(schemas(features::FeaturesResponse)),
    tags(
        (name = "health", description = "Health and feature endpoints"),
    ),
)]
struct CoreApiDoc;

#[cfg(any(
    feature = "converter-cbor",
    feature = "converter-json",
    feature = "converter-msgpack",
    feature = "converter-xml",
))]
#[derive(OpenApiMacro)]
#[openapi(
    paths(display_circuit::handler, simplify_circuit::handler, convert_circuit::handler),
    components(schemas(
        display_circuit::DisplayCircuitRequest,
        simplify_circuit::SimplifyCircuitRequest,
        simplify_circuit::SimplifyCircuitResponse,
        convert_circuit::ConvertCircuitRequest,
        convert_circuit::ConvertCircuitResponse,
        schema::Base64Circuit,
        schema::Circuit,
        schema::CircuitPayload,
        schema::CodeGenerationTargetName,
        schema::ConversionFormatName,
        schema::DiracFormatName,
        schema::DisplayFormatName,
        schema::GateName,
        schema::GateOperation,
        schema::PiFormatName,
        schema::PresentationFormatName,
    )),
    tags(
        (name = "circuit", description = "Circuit operations"),
        (name = "conversion", description = "Circuit format conversion"),
    ),
)]
struct ConverterApiDoc;

#[cfg(all(
    any(feature = "codegen-qiskit", feature = "codegen-openqasm3"),
    any(
        feature = "converter-cbor",
        feature = "converter-json",
        feature = "converter-msgpack",
        feature = "converter-xml",
    ),
))]
#[derive(OpenApiMacro)]
#[openapi(
    paths(generate_code::handler),
    components(schemas(
        generate_code::GenerateCodeRequest,
        generate_code::GenerateCodeResponse,
    )),
    tags(
        (name = "codegen", description = "Code generation"),
    ),
)]
struct CodegenApiDoc;

#[cfg(all(
    feature = "presenter-graphviz",
    any(
        feature = "converter-cbor",
        feature = "converter-json",
        feature = "converter-msgpack",
        feature = "converter-xml",
    ),
))]
#[derive(OpenApiMacro)]
#[openapi(
    paths(present_circuit::handler),
    components(schemas(
        present_circuit::PresentCircuitRequest,
    )),
    tags(
        (name = "presentation", description = "Circuit visualization"),
    ),
)]
struct PresenterApiDoc;

#[cfg(feature = "estimator")]
#[derive(OpenApiMacro)]
#[openapi(
    paths(estimate_circuit::handler),
    components(schemas(
        estimate_circuit::EstimateCircuitRequest,
        estimate_circuit::EstimateCircuitResponse,
    )),
    tags(
        (name = "estimator", description = "Circuit cost estimation"),
    ),
)]
struct EstimatorApiDoc;

pub(crate) fn build() -> OpenApi {
    let mut spec = CoreApiDoc::openapi();

    #[cfg(any(
        feature = "converter-cbor",
        feature = "converter-json",
        feature = "converter-msgpack",
        feature = "converter-xml",
    ))]
    {
        spec.merge(ConverterApiDoc::openapi());
    }

    #[cfg(all(
        any(feature = "codegen-qiskit", feature = "codegen-openqasm3"),
        any(
            feature = "converter-cbor",
            feature = "converter-json",
            feature = "converter-msgpack",
            feature = "converter-xml",
        ),
    ))]
    {
        spec.merge(CodegenApiDoc::openapi());
    }

    #[cfg(all(
        feature = "presenter-graphviz",
        any(
            feature = "converter-cbor",
            feature = "converter-json",
            feature = "converter-msgpack",
            feature = "converter-xml",
        ),
    ))]
    {
        spec.merge(PresenterApiDoc::openapi());
    }

    #[cfg(feature = "estimator")]
    {
        spec.merge(EstimatorApiDoc::openapi());
    }

    spec
}
