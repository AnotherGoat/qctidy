use utoipa::OpenApi as OpenApiMacro;
use utoipa::openapi::OpenApi;

use crate::features;
use crate::health;

#[cfg(any(
    feature = "converter-cbor",
    feature = "converter-json",
    feature = "converter-msgpack",
    feature = "converter-xml",
))]
use crate::{convert, display, simplify};

#[cfg(all(
    any(feature = "codegen-qiskit", feature = "codegen-openqasm3"),
    any(
        feature = "converter-cbor",
        feature = "converter-json",
        feature = "converter-msgpack",
        feature = "converter-xml",
    ),
))]
use crate::codegen;

#[cfg(all(
    feature = "presenter-graphviz",
    any(
        feature = "converter-cbor",
        feature = "converter-json",
        feature = "converter-msgpack",
        feature = "converter-xml",
    ),
))]
use crate::present;

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
    paths(display::handler, simplify::handler, convert::handler),
    components(schemas(
        display::DisplayRequestBody,
        simplify::SimplifyRequestBody,
        simplify::SimplifyResponseBody,
        convert::ConvertRequestBody,
        convert::ConvertResponseBody,
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
    paths(codegen::handler),
    components(schemas(
        codegen::CodegenRequestBody,
        codegen::CodegenResponseBody,
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
    paths(present::handler),
    components(schemas(
        present::PresentRequestBody,
    )),
    tags(
        (name = "presentation", description = "Circuit visualization"),
    ),
)]
struct PresenterApiDoc;

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

    spec
}
