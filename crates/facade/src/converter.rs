use std::sync::Arc;

use getset::{CloneGetters, CopyGetters};
use inew::New;
use qsimplify::GateOperation;
use qsimplify_ports::{ConversionFormat, ConverterPort, ParseError, SerializeError};

#[derive(Debug, Clone, CloneGetters, CopyGetters, New)]
#[new(pub, const)]
#[must_use]
pub struct ParseRequest {
    #[get_clone = "pub"]
    input: Arc<[u8]>,
    #[get_copy = "pub"]
    format: ConversionFormat,
}

#[derive(Debug, Clone, CloneGetters, New)]
#[new(pub, const)]
#[must_use]
pub struct ParseResponse {
    #[get_clone = "pub"]
    operations: Arc<[GateOperation]>,
}

#[derive(Debug, Clone, CloneGetters, CopyGetters, New)]
#[new(pub, const)]
#[must_use]
pub struct SerializeRequest {
    #[get_clone = "pub"]
    operations: Arc<[GateOperation]>,
    #[get_copy = "pub"]
    format: ConversionFormat,
    #[get_copy = "pub"]
    prettify: Option<bool>,
    #[get_copy = "pub"]
    indentation: Option<usize>,
}

#[derive(Debug, Clone, CloneGetters, New)]
#[new(pub, const)]
#[must_use]
pub struct SerializeResponse {
    #[get_clone = "pub"]
    bytes: Arc<[u8]>,
}

pub fn parse<C: ConverterPort>(
    request: &ParseRequest,
    converter: &C,
) -> Result<ParseResponse, ParseError> {
    converter
        .parse(&request.input(), request.format())
        .map(|operations| ParseResponse::new(operations.into()))
}

pub fn serialize<C: ConverterPort>(
    request: &SerializeRequest,
    converter: &C,
) -> Result<SerializeResponse, SerializeError> {
    converter
        .serialize(
            &request.operations(),
            request.format(),
            request.prettify(),
            request.indentation(),
        )
        .map(|bytes| SerializeResponse::new(bytes.into()))
}
