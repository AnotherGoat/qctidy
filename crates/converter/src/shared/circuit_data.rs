use qsimplify::{Circuit, GateOperation};
use qsimplify_ports::ParseError;

use crate::{CURRENT_FORMAT_VERSION, shared::gate_operation_data::GateOperationData};

pub(crate) struct CircuitData {
    pub version: Option<u16>,
    pub qubit_count: usize,
    pub operations: Vec<GateOperationData>,
}

impl From<&Circuit> for CircuitData {
    fn from(circuit: &Circuit) -> Self {
        Self {
            version: Some(CURRENT_FORMAT_VERSION),
            qubit_count: circuit.qubit_count(),
            operations: circuit
                .operations()
                .iter()
                .map(GateOperationData::from)
                .collect(),
        }
    }
}

impl TryFrom<CircuitData> for Circuit {
    type Error = ParseError;

    fn try_from(data: CircuitData) -> Result<Self, Self::Error> {
        let version = data.version.unwrap_or(CURRENT_FORMAT_VERSION);

        match version {
            1 => {}
            _ => {
                return Err(ParseError::UnsupportedVersion { version });
            }
        }

        let operations = data
            .operations
            .into_iter()
            .map(GateOperation::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self::new(data.qubit_count, operations))
    }
}
