use o2o::o2o;
use qsimplify::Circuit;
use qsimplify_ports::{ConversionFormat, ParseError, SerializeError};
use serde::{Deserialize, Serialize};
use serde_json::{
    Error,
    ser::{PrettyFormatter, Serializer},
};
use serde_with::skip_serializing_none;

use crate::shared::{circuit_data::CircuitData, gate_operation_data::GateOperationData};

#[skip_serializing_none]
#[derive(Serialize, Deserialize, o2o)]
#[serde(deny_unknown_fields)]
#[from_owned(CircuitData)]
#[owned_into(CircuitData)]
struct JsonCircuitData {
    version: Option<u16>,
    qubit_count: usize,
    #[map(operations, ~.into_iter().map(|operation| operation.into()).collect())]
    operations: Vec<JsonGateOperationData>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, o2o)]
#[serde(deny_unknown_fields)]
#[map_owned(GateOperationData)]
struct JsonGateOperationData {
    gate: String,
    qubit: Option<usize>,
    qubit1: Option<usize>,
    qubit2: Option<usize>,
    qubit3: Option<usize>,
    control: Option<usize>,
    control1: Option<usize>,
    control2: Option<usize>,
    target: Option<usize>,
    target1: Option<usize>,
    target2: Option<usize>,
    angle: Option<f64>,
    bit: Option<usize>,
}

pub fn parse(input: &[u8]) -> Result<Circuit, ParseError> {
    let circuit_data: JsonCircuitData =
        serde_json::from_slice(input).map_err(|error| map_json_error(&error, "circuit"))?;
    let generic: CircuitData = circuit_data.into();

    Circuit::try_from(generic)
}

fn map_json_error(error: &Error, gate_hint: &str) -> ParseError {
    let message = error.to_string();

    if message.starts_with("unknown field") {
        let field = message
            .split('`')
            .nth(1)
            .map(ToOwned::to_owned)
            .unwrap_or_default();

        return ParseError::UnknownField {
            field,
            gate: gate_hint.to_owned(),
        };
    }

    ParseError::InvalidInput {
        format: ConversionFormat::Json,
        message,
    }
}
pub fn serialize(
    circuit: &Circuit,
    prettify: bool,
    indentation: usize,
) -> Result<Vec<u8>, SerializeError> {
    let data = JsonCircuitData::from(CircuitData::from(circuit));

    if !prettify {
        return serde_json::to_vec(&data).map_err(|error| SerializeError::SerializationFailure {
            format: ConversionFormat::Json,
            message: error.to_string(),
        });
    }

    let width = indentation.max(1);
    let indent = vec![b' '; width];

    let mut buffer = Vec::new();
    let formatter = PrettyFormatter::with_indent(&indent);
    let mut serializer = Serializer::with_formatter(&mut buffer, formatter);

    data.serialize(&mut serializer)
        .map_err(|error| SerializeError::SerializationFailure {
            format: ConversionFormat::Json,
            message: error.to_string(),
        })?;

    Ok(buffer)
}
