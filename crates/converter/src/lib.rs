use qsimplify::GateOperation;
use qsimplify_ports::{ConversionFormat, ConverterPort, ParseError, SerializeError};

pub mod json;
pub mod message_pack;

#[cfg(test)]
mod json_tests;

#[cfg(test)]
mod message_pack_tests;

#[derive(Debug, Clone, Copy)]
pub struct ConverterAdapter;

#[expect(clippy::unimplemented)]
impl ConverterPort for ConverterAdapter {
    fn parse(
        &self,
        input: &[u8],
        format: ConversionFormat,
    ) -> Result<Vec<GateOperation>, ParseError> {
        use ConversionFormat::*;

        match format {
            Json => json::parse(input),
            MessagePack => message_pack::parse(input),
            Binary => unimplemented!(),
            Base64 => unimplemented!(),
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
            MessagePack => message_pack::serialize(operations),
            Binary => unimplemented!(),
            Base64 => unimplemented!(),
        }
    }
}
