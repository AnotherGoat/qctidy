use qsimplify::dto::GateOperation;
use qsimplify_ports::{ConversionFormat, ConverterPort, ParseError, SerializeError};

pub mod json;

#[cfg(test)]
mod json_tests;

#[derive(Debug, Clone, Copy)]
pub struct ConverterAdapter;

impl ConverterPort for ConverterAdapter {
    fn parse(
        &self,
        input: &[u8],
        format: ConversionFormat,
    ) -> Result<Vec<GateOperation>, ParseError> {
        match format {
            ConversionFormat::Json => json::parse(input),
            ConversionFormat::Binary => todo!(),
            ConversionFormat::Base64 => todo!(),
        }
    }

    fn serialize(
        &self,
        operations: &[GateOperation],
        format: ConversionFormat,
        prettify: Option<bool>,
        indentation: Option<usize>,
    ) -> Result<Vec<u8>, SerializeError> {
        use ConversionFormat::*;

        match format {
            Json => json::serialize(
                operations,
                prettify.unwrap_or(false),
                indentation.unwrap_or(2),
            ),
            Binary => todo!(),
            Base64 => todo!(),
        }
    }
}
