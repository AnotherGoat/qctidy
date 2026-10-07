use std::sync::Arc;

use getset::{CloneGetters, Getters};
use newgen::New;
use qctidy_ports::{PythonAnalysis, PythonPort};

#[derive(Debug, Clone, CloneGetters, New)]
#[new(pub, const)]
#[must_use]
pub struct ParsePythonRequest {
    /// The Python source to parse.
    #[get_clone = "pub"]
    source: Arc<str>,
}

#[derive(Debug, Clone, Getters, New)]
#[new(pub, const)]
#[must_use]
pub struct ParsePythonResponse {
    /// The circuits and scopes found in the source.
    #[get = "pub"]
    analysis: PythonAnalysis,
}

/// Parse Python source into the circuits and scopes it defines.
pub fn parse_python<P: PythonPort>(
    request: &ParsePythonRequest,
    parser: &P,
) -> ParsePythonResponse {
    ParsePythonResponse::new(parser.parse(&request.source()))
}
