use qsimplify::dto::GateOperation;
use qsimplify_ports::{ParseError, SerializeError};
use serde::{Deserialize, Serialize};
use serde_json::{
    Error,
    ser::{PrettyFormatter, Serializer},
};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct GateOperationData {
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

impl GateOperationData {
    const fn new(gate: String) -> Self {
        Self {
            gate,
            qubit: None,
            qubit1: None,
            qubit2: None,
            qubit3: None,
            control: None,
            control1: None,
            control2: None,
            target: None,
            target1: None,
            target2: None,
            angle: None,
            bit: None,
        }
    }

    const fn qubit(mut self, qubit: usize) -> Self {
        self.qubit = Some(qubit);
        self
    }

    const fn qubit1(mut self, qubit1: usize) -> Self {
        self.qubit1 = Some(qubit1);
        self
    }

    const fn qubit2(mut self, qubit2: usize) -> Self {
        self.qubit2 = Some(qubit2);
        self
    }

    const fn qubit3(mut self, qubit3: usize) -> Self {
        self.qubit3 = Some(qubit3);
        self
    }

    const fn control(mut self, control: usize) -> Self {
        self.control = Some(control);
        self
    }

    const fn control1(mut self, control1: usize) -> Self {
        self.control1 = Some(control1);
        self
    }

    const fn control2(mut self, control2: usize) -> Self {
        self.control2 = Some(control2);
        self
    }

    const fn target(mut self, target: usize) -> Self {
        self.target = Some(target);
        self
    }

    const fn target1(mut self, target1: usize) -> Self {
        self.target1 = Some(target1);
        self
    }

    const fn target2(mut self, target2: usize) -> Self {
        self.target2 = Some(target2);
        self
    }

    const fn angle(mut self, angle: f64) -> Self {
        self.angle = Some(angle);
        self
    }

    const fn bit(mut self, bit: usize) -> Self {
        self.bit = Some(bit);
        self
    }
}

impl From<&GateOperation> for GateOperationData {
    fn from(operation: &GateOperation) -> Self {
        use GateOperation::*;

        let gate = operation.r#type().to_string();

        match *operation {
            ID { qubit }
            | H { qubit }
            | X { qubit }
            | Y { qubit }
            | Z { qubit }
            | S { qubit }
            | SDG { qubit }
            | SX { qubit }
            | SY { qubit }
            | T { qubit }
            | TDG { qubit } => Self::new(gate).qubit(qubit),
            P { angle, qubit }
            | RX { angle, qubit }
            | RY { angle, qubit }
            | RZ { angle, qubit } => Self::new(gate).qubit(qubit).angle(angle),
            Measure { qubit, bit } => Self::new(gate).qubit(qubit).bit(bit),
            Swap { qubit1, qubit2 } | CZ { qubit1, qubit2 } => {
                Self::new(gate).qubit1(qubit1).qubit2(qubit2)
            }
            CH { control, target } | CX { control, target } | CY { control, target } => {
                Self::new(gate).control(control).target(target)
            }
            CP {
                angle,
                qubit1,
                qubit2,
            } => Self::new(gate).qubit1(qubit1).qubit2(qubit2).angle(angle),
            CSwap {
                control,
                target1,
                target2,
            } => Self::new(gate)
                .control(control)
                .target1(target1)
                .target2(target2),
            CCX {
                control1,
                control2,
                target,
            } => Self::new(gate)
                .control1(control1)
                .control2(control2)
                .target(target),
            CCZ {
                qubit1,
                qubit2,
                qubit3,
            } => Self::new(gate).qubit1(qubit1).qubit2(qubit2).qubit3(qubit3),
        }
    }
}

impl TryFrom<GateOperationData> for GateOperation {
    type Error = ParseError;

    fn try_from(data: GateOperationData) -> Result<Self, Self::Error> {
        use GateOperation::*;

        let gate = data.gate.as_str();
        let missing_field = |field: &str| ParseError::MissingRequiredJsonField {
            field: field.to_owned(),
            gate: gate.to_owned(),
        };

        match gate {
            "id" => Ok(ID {
                qubit: data.qubit.ok_or_else(|| missing_field("qubit"))?,
            }),
            "h" => Ok(H {
                qubit: data.qubit.ok_or_else(|| missing_field("qubit"))?,
            }),
            "x" => Ok(X {
                qubit: data.qubit.ok_or_else(|| missing_field("qubit"))?,
            }),
            "y" => Ok(Y {
                qubit: data.qubit.ok_or_else(|| missing_field("qubit"))?,
            }),
            "z" => Ok(Z {
                qubit: data.qubit.ok_or_else(|| missing_field("qubit"))?,
            }),
            "p" => Ok(P {
                qubit: data.qubit.ok_or_else(|| missing_field("qubit"))?,
                angle: data.angle.ok_or_else(|| missing_field("angle"))?,
            }),
            "rx" => Ok(RX {
                qubit: data.qubit.ok_or_else(|| missing_field("qubit"))?,
                angle: data.angle.ok_or_else(|| missing_field("angle"))?,
            }),
            "ry" => Ok(RY {
                qubit: data.qubit.ok_or_else(|| missing_field("qubit"))?,
                angle: data.angle.ok_or_else(|| missing_field("angle"))?,
            }),
            "rz" => Ok(RZ {
                qubit: data.qubit.ok_or_else(|| missing_field("qubit"))?,
                angle: data.angle.ok_or_else(|| missing_field("angle"))?,
            }),
            "s" => Ok(S {
                qubit: data.qubit.ok_or_else(|| missing_field("qubit"))?,
            }),
            "sdg" => Ok(SDG {
                qubit: data.qubit.ok_or_else(|| missing_field("qubit"))?,
            }),
            "sx" => Ok(SX {
                qubit: data.qubit.ok_or_else(|| missing_field("qubit"))?,
            }),
            "sy" => Ok(SY {
                qubit: data.qubit.ok_or_else(|| missing_field("qubit"))?,
            }),
            "t" => Ok(T {
                qubit: data.qubit.ok_or_else(|| missing_field("qubit"))?,
            }),
            "tdg" => Ok(TDG {
                qubit: data.qubit.ok_or_else(|| missing_field("qubit"))?,
            }),
            "m" => Ok(Measure {
                qubit: data.qubit.ok_or_else(|| missing_field("qubit"))?,
                bit: data.bit.ok_or_else(|| missing_field("bit"))?,
            }),
            "swap" => Ok(Swap {
                qubit1: data.qubit1.ok_or_else(|| missing_field("qubit1"))?,
                qubit2: data.qubit2.ok_or_else(|| missing_field("qubit2"))?,
            }),
            "ch" => Ok(CH {
                control: data.control.ok_or_else(|| missing_field("control"))?,
                target: data.target.ok_or_else(|| missing_field("target"))?,
            }),
            "cx" => Ok(CX {
                control: data.control.ok_or_else(|| missing_field("control"))?,
                target: data.target.ok_or_else(|| missing_field("target"))?,
            }),
            "cy" => Ok(CY {
                control: data.control.ok_or_else(|| missing_field("control"))?,
                target: data.target.ok_or_else(|| missing_field("target"))?,
            }),
            "cz" => Ok(CZ {
                qubit1: data.qubit1.ok_or_else(|| missing_field("qubit1"))?,
                qubit2: data.qubit2.ok_or_else(|| missing_field("qubit2"))?,
            }),
            "cp" => Ok(CP {
                qubit1: data.qubit1.ok_or_else(|| missing_field("qubit1"))?,
                qubit2: data.qubit2.ok_or_else(|| missing_field("qubit2"))?,
                angle: data.angle.ok_or_else(|| missing_field("angle"))?,
            }),
            "cswap" => Ok(CSwap {
                control: data.control.ok_or_else(|| missing_field("control"))?,
                target1: data.target1.ok_or_else(|| missing_field("target1"))?,
                target2: data.target2.ok_or_else(|| missing_field("target2"))?,
            }),
            "ccx" => Ok(CCX {
                control1: data.control1.ok_or_else(|| missing_field("control1"))?,
                control2: data.control2.ok_or_else(|| missing_field("control2"))?,
                target: data.target.ok_or_else(|| missing_field("target"))?,
            }),
            "ccz" => Ok(CCZ {
                qubit1: data.qubit1.ok_or_else(|| missing_field("qubit1"))?,
                qubit2: data.qubit2.ok_or_else(|| missing_field("qubit2"))?,
                qubit3: data.qubit3.ok_or_else(|| missing_field("qubit3"))?,
            }),
            _ => Err(ParseError::UnknownGateType {
                gate: gate.to_owned(),
            }),
        }
    }
}

pub(crate) fn parse(input: &[u8]) -> Result<Vec<GateOperation>, ParseError> {
    let raw: Vec<serde_json::Value> =
        serde_json::from_slice(input).map_err(|error| ParseError::InvalidJson {
            message: error.to_string(),
        })?;

    raw.into_iter()
        .map(|value| {
            let gate_hint = value
                .get("gate")
                .and_then(|v| v.as_str())
                .unwrap_or("unknown")
                .to_owned();

            let data: GateOperationData = serde_json::from_value(value)
                .map_err(|error| map_serde_error(&error, &gate_hint))?;

            GateOperation::try_from(data)
        })
        .collect()
}

fn map_serde_error(error: &Error, gate_hint: &str) -> ParseError {
    let message = error.to_string();
    if message.starts_with("unknown field") {
        let field = message
            .split('`')
            .nth(1)
            .map(ToOwned::to_owned)
            .unwrap_or_default();
        return ParseError::UnknownJsonField {
            field,
            gate: gate_hint.to_owned(),
        };
    }
    ParseError::InvalidJson { message }
}

pub(crate) fn serialize(
    operations: &[GateOperation],
    prettify: bool,
    indentation: usize,
) -> Result<Vec<u8>, SerializeError> {
    let datas: Vec<GateOperationData> = operations.iter().map(GateOperationData::from).collect();

    if !prettify {
        return serde_json::to_vec(&datas).map_err(|error| SerializeError::JsonSerializationFail {
            message: error.to_string(),
        });
    }

    let width = indentation.max(1);
    let indent = vec![b' '; width];

    let mut buffer = Vec::new();
    let formatter = PrettyFormatter::with_indent(&indent);
    let mut serializer = Serializer::with_formatter(&mut buffer, formatter);

    datas
        .serialize(&mut serializer)
        .map_err(|error| SerializeError::JsonSerializationFail {
            message: error.to_string(),
        })?;

    Ok(buffer)
}
