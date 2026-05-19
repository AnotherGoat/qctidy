#[cfg(feature = "converter-cbor")]
pub(crate) mod cbor;
#[cfg(feature = "converter-json")]
pub(crate) mod json;
#[cfg(feature = "converter-msgpack")]
pub(crate) mod message_pack;
#[cfg(feature = "converter-xml")]
pub(crate) mod xml;

#[cfg(test)]
#[cfg(feature = "converter-cbor")]
mod cbor_tests;
#[cfg(test)]
#[cfg(feature = "converter-json")]
mod json_tests;
#[cfg(test)]
#[cfg(feature = "converter-msgpack")]
mod message_pack_tests;
#[cfg(test)]
#[cfg(feature = "converter-xml")]
mod xml_tests;
