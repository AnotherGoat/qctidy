#[cfg(any(feature = "converter-cbor", feature = "converter-msgpack"))]
pub(crate) mod binary_gate_type;
pub(crate) mod circuit_data;
pub(crate) mod gate_operation_data;
#[cfg(feature = "converter-json")]
pub(crate) mod readable_gate_type;
