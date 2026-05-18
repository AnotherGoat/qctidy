use qsimplify::Circuit;
use qsimplify_ports::{ConversionFormat, ConverterPort, ParseError, SerializeError};

use crate::formats::{cbor, json, message_pack, xml};

mod formats;
pub(crate) mod shared;

pub use formats::cbor::{parse as parse_cbor, serialize as serialize_cbor};
pub use formats::json::{parse as parse_json, serialize as serialize_json};
pub use formats::message_pack::{parse as parse_msgpack, serialize as serialize_msgpack};
pub use formats::xml::{parse as parse_xml, serialize as serialize_xml};

/// Current format version used for parsing and serializing, for any `ConversionFormat`.
pub const CURRENT_FORMAT_VERSION: u16 = 1;

#[derive(Debug, Clone, Copy)]
pub struct ConverterAdapter;

impl ConverterPort for ConverterAdapter {
    fn parse(&self, input: &[u8], format: ConversionFormat) -> Result<Circuit, ParseError> {
        use ConversionFormat::*;

        match format {
            Json => json::parse(input),
            Xml => xml::parse(input),
            MessagePack => message_pack::parse(input),
            Cbor => cbor::parse(input),
        }
    }

    fn serialize(
        &self,
        circuit: &Circuit,
        format: ConversionFormat,
        prettify: Option<bool>,
        indentation: Option<usize>,
    ) -> Result<Vec<u8>, SerializeError> {
        use ConversionFormat::*;

        match format {
            Json => json::serialize(circuit, prettify.unwrap_or(false), indentation.unwrap_or(2)),
            Xml => xml::serialize(circuit, prettify.unwrap_or(false), indentation.unwrap_or(2)),
            MessagePack => message_pack::serialize(circuit),
            Cbor => cbor::serialize(circuit),
        }
    }
}
