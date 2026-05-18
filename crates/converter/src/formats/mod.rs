pub(crate) mod cbor;
pub(crate) mod json;
pub(crate) mod message_pack;
pub(crate) mod xml;

#[cfg(test)]
mod cbor_tests;
#[cfg(test)]
mod json_tests;
#[cfg(test)]
mod message_pack_tests;
#[cfg(test)]
mod xml_tests;
