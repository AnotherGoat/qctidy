use qctidy::Circuit;
use qctidy_ports::{ConversionFormat, ConverterPort, ParseError, SerializeError};

#[cfg(feature = "converter-cbor")]
use formats::cbor;
#[cfg(feature = "converter-json")]
use formats::json;
#[cfg(feature = "converter-msgpack")]
use formats::message_pack;
#[cfg(feature = "converter-xml")]
use formats::xml;

mod formats;
pub(crate) mod shared;

#[cfg(feature = "converter-cbor")]
pub use formats::cbor::{parse as parse_cbor, serialize as serialize_cbor};
#[cfg(feature = "converter-json")]
pub use formats::json::{parse as parse_json, serialize as serialize_json};
#[cfg(feature = "converter-msgpack")]
pub use formats::message_pack::{parse as parse_msgpack, serialize as serialize_msgpack};
#[cfg(feature = "converter-xml")]
pub use formats::xml::{parse as parse_xml, serialize as serialize_xml};

/// Current format version used for parsing and serializing, for any `ConversionFormat`.
pub const CURRENT_FORMAT_VERSION: u16 = 1;

#[derive(Debug, Clone, Copy)]
pub struct ConverterAdapter;

impl ConverterPort for ConverterAdapter {
    fn parse(&self, input: &[u8], format: ConversionFormat) -> Result<Circuit, ParseError> {
        use ConversionFormat::*;

        match format {
            Json => {
                #[cfg(feature = "converter-json")]
                {
                    json::parse(input)
                }
                #[cfg(not(feature = "converter-json"))]
                {
                    Err(ParseError::MissingFeature {
                        feature: "converter-json",
                        format,
                    })
                }
            }
            Xml => {
                #[cfg(feature = "converter-xml")]
                {
                    xml::parse(input)
                }
                #[cfg(not(feature = "converter-xml"))]
                {
                    Err(ParseError::MissingFeature {
                        feature: "converter-xml",
                        format,
                    })
                }
            }
            MessagePack => {
                #[cfg(feature = "converter-msgpack")]
                {
                    message_pack::parse(input)
                }
                #[cfg(not(feature = "converter-msgpack"))]
                {
                    Err(ParseError::MissingFeature {
                        feature: "converter-msgpack",
                        format,
                    })
                }
            }
            Cbor => {
                #[cfg(feature = "converter-cbor")]
                {
                    cbor::parse(input)
                }
                #[cfg(not(feature = "converter-cbor"))]
                {
                    Err(ParseError::MissingFeature {
                        feature: "converter-cbor",
                        format,
                    })
                }
            }
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
            Json => {
                #[cfg(feature = "converter-json")]
                {
                    json::serialize(circuit, prettify.unwrap_or(false), indentation.unwrap_or(2))
                }
                #[cfg(not(feature = "converter-json"))]
                {
                    Err(SerializeError::MissingFeature {
                        feature: "converter-json",
                        format,
                    })
                }
            }
            Xml => {
                #[cfg(feature = "converter-xml")]
                {
                    xml::serialize(circuit, prettify.unwrap_or(false), indentation.unwrap_or(2))
                }
                #[cfg(not(feature = "converter-xml"))]
                {
                    Err(SerializeError::MissingFeature {
                        feature: "converter-xml",
                        format,
                    })
                }
            }
            MessagePack => {
                #[cfg(feature = "converter-msgpack")]
                {
                    message_pack::serialize(circuit)
                }
                #[cfg(not(feature = "converter-msgpack"))]
                {
                    Err(SerializeError::MissingFeature {
                        feature: "converter-msgpack",
                        format,
                    })
                }
            }
            Cbor => {
                #[cfg(feature = "converter-cbor")]
                {
                    cbor::serialize(circuit)
                }
                #[cfg(not(feature = "converter-cbor"))]
                {
                    Err(SerializeError::MissingFeature {
                        feature: "converter-cbor",
                        format,
                    })
                }
            }
        }
    }
}
