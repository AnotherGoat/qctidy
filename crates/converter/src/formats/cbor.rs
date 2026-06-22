use std::io;

use ciborium::de::{self, from_reader};
use ciborium::ser::into_writer;
use o2o::o2o;
use qsimplify::Circuit;
use qsimplify_ports::{ConversionFormat, ParseError, SerializeError};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::shared::binary_gate_type::BinaryGateType;
use crate::shared::circuit_data::CircuitData;
use crate::shared::gate_operation_data::GateOperationData;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, o2o)]
#[serde(deny_unknown_fields)]
#[from_owned(CircuitData)]
#[owned_into(CircuitData)]
struct CborCircuitData {
    #[serde(rename = "v")]
    version: Option<u16>,
    #[serde(rename = "q")]
    qubit_count: usize,
    #[serde(rename = "o")]
    #[map(operations, ~.into_iter().map(Into::into).collect())]
    operations: Vec<CborGateOperationData>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, o2o)]
#[serde(deny_unknown_fields)]
#[map_owned(GateOperationData)]
struct CborGateOperationData {
    #[serde(rename = "g")]
    #[from(gate, BinaryGateType(~))]
    #[into(gate, ~.0)]
    gate: BinaryGateType,
    #[serde(rename = "q", default)]
    qubit: Option<usize>,
    #[serde(rename = "q1", default)]
    qubit1: Option<usize>,
    #[serde(rename = "q2", default)]
    qubit2: Option<usize>,
    #[serde(rename = "q3", default)]
    qubit3: Option<usize>,
    #[serde(rename = "c", default)]
    control: Option<usize>,
    #[serde(rename = "c1", default)]
    control1: Option<usize>,
    #[serde(rename = "c2", default)]
    control2: Option<usize>,
    #[serde(rename = "t", default)]
    target: Option<usize>,
    #[serde(rename = "t1", default)]
    target1: Option<usize>,
    #[serde(rename = "t2", default)]
    target2: Option<usize>,
    #[serde(rename = "a", default)]
    theta: Option<f64>,
    #[serde(rename = "a2", default)]
    phi: Option<f64>,
    #[serde(rename = "a3", default)]
    lambda: Option<f64>,
    #[serde(rename = "b", default)]
    bit: Option<usize>,
}

pub fn parse(input: &[u8]) -> Result<Circuit, ParseError> {
    let circuit_data: CborCircuitData =
        from_reader(input).map_err(|error| map_decode_error(&error))?;

    let generic: CircuitData = circuit_data.into();

    Circuit::try_from(generic)
}

fn map_decode_error(error: &de::Error<io::Error>) -> ParseError {
    let message = error.to_string();

    if message.contains("unknown field") {
        let field = message
            .split('`')
            .nth(1)
            .map(ToOwned::to_owned)
            .unwrap_or_default();

        return ParseError::UnknownField {
            field,
            gate: "unknown".to_owned(),
        };
    }

    ParseError::InvalidInput {
        format: ConversionFormat::Cbor,
        message,
    }
}

pub fn serialize(circuit: &Circuit) -> Result<Vec<u8>, SerializeError> {
    let data = CborCircuitData::from(CircuitData::from(circuit));

    let mut output = Vec::new();

    into_writer(&data, &mut output).map_err(|error| SerializeError::SerializationFailure {
        format: ConversionFormat::Cbor,
        message: error.to_string(),
    })?;

    Ok(output)
}
