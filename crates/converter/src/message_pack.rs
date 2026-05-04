use qsimplify::GateOperation;
use qsimplify_ports::{ConversionFormat, ParseError, SerializeError};
use rmp_serde::decode::{Error as DecodeError, from_slice};
use rmp_serde::encode::to_vec_named;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct GateOperationData {
    #[serde(rename = "g")]
    gate: String,
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
    angle: Option<f64>,
    #[serde(rename = "b", default)]
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
        let missing_field = |field: &str| ParseError::MissingRequiredField {
            format: ConversionFormat::MessagePack,
            field: field.to_owned(),
            gate: gate.to_owned(),
        };

        match gate {
            "id" => Ok(ID {
                qubit: data.qubit.ok_or_else(|| missing_field("q"))?,
            }),
            "h" => Ok(H {
                qubit: data.qubit.ok_or_else(|| missing_field("q"))?,
            }),
            "x" => Ok(X {
                qubit: data.qubit.ok_or_else(|| missing_field("q"))?,
            }),
            "y" => Ok(Y {
                qubit: data.qubit.ok_or_else(|| missing_field("q"))?,
            }),
            "z" => Ok(Z {
                qubit: data.qubit.ok_or_else(|| missing_field("q"))?,
            }),
            "p" => Ok(P {
                qubit: data.qubit.ok_or_else(|| missing_field("q"))?,
                angle: data.angle.ok_or_else(|| missing_field("a"))?,
            }),
            "rx" => Ok(RX {
                qubit: data.qubit.ok_or_else(|| missing_field("q"))?,
                angle: data.angle.ok_or_else(|| missing_field("a"))?,
            }),
            "ry" => Ok(RY {
                qubit: data.qubit.ok_or_else(|| missing_field("q"))?,
                angle: data.angle.ok_or_else(|| missing_field("a"))?,
            }),
            "rz" => Ok(RZ {
                qubit: data.qubit.ok_or_else(|| missing_field("q"))?,
                angle: data.angle.ok_or_else(|| missing_field("a"))?,
            }),
            "s" => Ok(S {
                qubit: data.qubit.ok_or_else(|| missing_field("q"))?,
            }),
            "sdg" => Ok(SDG {
                qubit: data.qubit.ok_or_else(|| missing_field("q"))?,
            }),
            "sx" => Ok(SX {
                qubit: data.qubit.ok_or_else(|| missing_field("q"))?,
            }),
            "sy" => Ok(SY {
                qubit: data.qubit.ok_or_else(|| missing_field("q"))?,
            }),
            "t" => Ok(T {
                qubit: data.qubit.ok_or_else(|| missing_field("q"))?,
            }),
            "tdg" => Ok(TDG {
                qubit: data.qubit.ok_or_else(|| missing_field("q"))?,
            }),
            "m" => Ok(Measure {
                qubit: data.qubit.ok_or_else(|| missing_field("q"))?,
                bit: data.bit.ok_or_else(|| missing_field("b"))?,
            }),
            "swap" => Ok(Swap {
                qubit1: data.qubit1.ok_or_else(|| missing_field("q1"))?,
                qubit2: data.qubit2.ok_or_else(|| missing_field("q2"))?,
            }),
            "ch" => Ok(CH {
                control: data.control.ok_or_else(|| missing_field("c"))?,
                target: data.target.ok_or_else(|| missing_field("t"))?,
            }),
            "cx" => Ok(CX {
                control: data.control.ok_or_else(|| missing_field("c"))?,
                target: data.target.ok_or_else(|| missing_field("t"))?,
            }),
            "cy" => Ok(CY {
                control: data.control.ok_or_else(|| missing_field("c"))?,
                target: data.target.ok_or_else(|| missing_field("t"))?,
            }),
            "cz" => Ok(CZ {
                qubit1: data.qubit1.ok_or_else(|| missing_field("q1"))?,
                qubit2: data.qubit2.ok_or_else(|| missing_field("q2"))?,
            }),
            "cp" => Ok(CP {
                qubit1: data.qubit1.ok_or_else(|| missing_field("q1"))?,
                qubit2: data.qubit2.ok_or_else(|| missing_field("q2"))?,
                angle: data.angle.ok_or_else(|| missing_field("a"))?,
            }),
            "cswap" => Ok(CSwap {
                control: data.control.ok_or_else(|| missing_field("c"))?,
                target1: data.target1.ok_or_else(|| missing_field("t1"))?,
                target2: data.target2.ok_or_else(|| missing_field("t2"))?,
            }),
            "ccx" => Ok(CCX {
                control1: data.control1.ok_or_else(|| missing_field("c1"))?,
                control2: data.control2.ok_or_else(|| missing_field("c2"))?,
                target: data.target.ok_or_else(|| missing_field("t"))?,
            }),
            "ccz" => Ok(CCZ {
                qubit1: data.qubit1.ok_or_else(|| missing_field("q1"))?,
                qubit2: data.qubit2.ok_or_else(|| missing_field("q2"))?,
                qubit3: data.qubit3.ok_or_else(|| missing_field("q3"))?,
            }),
            _ => Err(ParseError::UnknownGateType {
                gate: gate.to_owned(),
            }),
        }
    }
}

pub(crate) fn parse(input: &[u8]) -> Result<Vec<GateOperation>, ParseError> {
    let raw: Vec<GateOperationData> =
        from_slice(input).map_err(|error| map_decode_error(&error))?;

    raw.into_iter().map(GateOperation::try_from).collect()
}

fn map_decode_error(error: &DecodeError) -> ParseError {
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
        format: ConversionFormat::MessagePack,
        message,
    }
}

pub(crate) fn serialize(operations: &[GateOperation]) -> Result<Vec<u8>, SerializeError> {
    let datas: Vec<GateOperationData> = operations.iter().map(GateOperationData::from).collect();

    to_vec_named(&datas).map_err(|error| SerializeError::SerializationFailure {
        format: ConversionFormat::MessagePack,
        message: error.to_string(),
    })
}
