use qsimplify::dto::GateOperation;
use qsimplify_ports::{ConversionFormat, ParseError, SerializeError};
use rmp_serde::decode::{Error as DecodeError, from_slice};
use rmp_serde::encode::to_vec;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct GateOperationData {
    g: String,
    #[serde(default)]
    q: Option<usize>,
    #[serde(default)]
    q1: Option<usize>,
    #[serde(default)]
    q2: Option<usize>,
    #[serde(default)]
    q3: Option<usize>,
    #[serde(default)]
    c: Option<usize>,
    #[serde(default)]
    c1: Option<usize>,
    #[serde(default)]
    c2: Option<usize>,
    #[serde(default)]
    t: Option<usize>,
    #[serde(default)]
    t1: Option<usize>,
    #[serde(default)]
    t2: Option<usize>,
    #[serde(default)]
    a: Option<f64>,
    #[serde(default)]
    b: Option<usize>,
}

impl GateOperationData {
    const fn new(gate: String) -> Self {
        Self {
            g: gate,
            q: None,
            q1: None,
            q2: None,
            q3: None,
            c: None,
            c1: None,
            c2: None,
            t: None,
            t1: None,
            t2: None,
            a: None,
            b: None,
        }
    }

    const fn q(mut self, qubit: usize) -> Self {
        self.q = Some(qubit);
        self
    }

    const fn q1(mut self, qubit1: usize) -> Self {
        self.q1 = Some(qubit1);
        self
    }

    const fn q2(mut self, qubit2: usize) -> Self {
        self.q2 = Some(qubit2);
        self
    }

    const fn q3(mut self, qubit3: usize) -> Self {
        self.q3 = Some(qubit3);
        self
    }

    const fn c(mut self, control: usize) -> Self {
        self.c = Some(control);
        self
    }

    const fn c1(mut self, control1: usize) -> Self {
        self.c1 = Some(control1);
        self
    }

    const fn c2(mut self, control2: usize) -> Self {
        self.c2 = Some(control2);
        self
    }

    const fn t(mut self, target: usize) -> Self {
        self.t = Some(target);
        self
    }

    const fn t1(mut self, target1: usize) -> Self {
        self.t1 = Some(target1);
        self
    }

    const fn t2(mut self, target2: usize) -> Self {
        self.t2 = Some(target2);
        self
    }

    const fn a(mut self, angle: f64) -> Self {
        self.a = Some(angle);
        self
    }

    const fn b(mut self, bit: usize) -> Self {
        self.b = Some(bit);
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
            | TDG { qubit } => Self::new(gate).q(qubit),
            P { angle, qubit }
            | RX { angle, qubit }
            | RY { angle, qubit }
            | RZ { angle, qubit } => Self::new(gate).q(qubit).a(angle),
            Measure { qubit, bit } => Self::new(gate).q(qubit).b(bit),
            Swap { qubit1, qubit2 } | CZ { qubit1, qubit2 } => {
                Self::new(gate).q1(qubit1).q2(qubit2)
            }
            CH { control, target } | CX { control, target } | CY { control, target } => {
                Self::new(gate).c(control).t(target)
            }
            CP {
                angle,
                qubit1,
                qubit2,
            } => Self::new(gate).q1(qubit1).q2(qubit2).a(angle),
            CSwap {
                control,
                target1,
                target2,
            } => Self::new(gate).c(control).t1(target1).t2(target2),
            CCX {
                control1,
                control2,
                target,
            } => Self::new(gate).c1(control1).c2(control2).t(target),
            CCZ {
                qubit1,
                qubit2,
                qubit3,
            } => Self::new(gate).q1(qubit1).q2(qubit2).q3(qubit3),
        }
    }
}

impl TryFrom<GateOperationData> for GateOperation {
    type Error = ParseError;

    fn try_from(data: GateOperationData) -> Result<Self, Self::Error> {
        use GateOperation::*;

        let gate = data.g.as_str();
        let missing_field = |field: &str| ParseError::MissingRequiredField {
            format: ConversionFormat::MessagePack,
            field: field.to_owned(),
            gate: gate.to_owned(),
        };

        match gate {
            "id" => Ok(ID {
                qubit: data.q.ok_or_else(|| missing_field("q"))?,
            }),
            "h" => Ok(H {
                qubit: data.q.ok_or_else(|| missing_field("q"))?,
            }),
            "x" => Ok(X {
                qubit: data.q.ok_or_else(|| missing_field("q"))?,
            }),
            "y" => Ok(Y {
                qubit: data.q.ok_or_else(|| missing_field("q"))?,
            }),
            "z" => Ok(Z {
                qubit: data.q.ok_or_else(|| missing_field("q"))?,
            }),
            "p" => Ok(P {
                qubit: data.q.ok_or_else(|| missing_field("q"))?,
                angle: data.a.ok_or_else(|| missing_field("a"))?,
            }),
            "rx" => Ok(RX {
                qubit: data.q.ok_or_else(|| missing_field("q"))?,
                angle: data.a.ok_or_else(|| missing_field("a"))?,
            }),
            "ry" => Ok(RY {
                qubit: data.q.ok_or_else(|| missing_field("q"))?,
                angle: data.a.ok_or_else(|| missing_field("a"))?,
            }),
            "rz" => Ok(RZ {
                qubit: data.q.ok_or_else(|| missing_field("q"))?,
                angle: data.a.ok_or_else(|| missing_field("a"))?,
            }),
            "s" => Ok(S {
                qubit: data.q.ok_or_else(|| missing_field("q"))?,
            }),
            "sdg" => Ok(SDG {
                qubit: data.q.ok_or_else(|| missing_field("q"))?,
            }),
            "sx" => Ok(SX {
                qubit: data.q.ok_or_else(|| missing_field("q"))?,
            }),
            "sy" => Ok(SY {
                qubit: data.q.ok_or_else(|| missing_field("q"))?,
            }),
            "t" => Ok(T {
                qubit: data.q.ok_or_else(|| missing_field("q"))?,
            }),
            "tdg" => Ok(TDG {
                qubit: data.q.ok_or_else(|| missing_field("q"))?,
            }),
            "m" => Ok(Measure {
                qubit: data.q.ok_or_else(|| missing_field("q"))?,
                bit: data.b.ok_or_else(|| missing_field("b"))?,
            }),
            "swap" => Ok(Swap {
                qubit1: data.q1.ok_or_else(|| missing_field("q1"))?,
                qubit2: data.q2.ok_or_else(|| missing_field("q2"))?,
            }),
            "ch" => Ok(CH {
                control: data.c.ok_or_else(|| missing_field("c"))?,
                target: data.t.ok_or_else(|| missing_field("t"))?,
            }),
            "cx" => Ok(CX {
                control: data.c.ok_or_else(|| missing_field("c"))?,
                target: data.t.ok_or_else(|| missing_field("t"))?,
            }),
            "cy" => Ok(CY {
                control: data.c.ok_or_else(|| missing_field("c"))?,
                target: data.t.ok_or_else(|| missing_field("t"))?,
            }),
            "cz" => Ok(CZ {
                qubit1: data.q1.ok_or_else(|| missing_field("q1"))?,
                qubit2: data.q2.ok_or_else(|| missing_field("q2"))?,
            }),
            "cp" => Ok(CP {
                qubit1: data.q1.ok_or_else(|| missing_field("q1"))?,
                qubit2: data.q2.ok_or_else(|| missing_field("q2"))?,
                angle: data.a.ok_or_else(|| missing_field("a"))?,
            }),
            "cswap" => Ok(CSwap {
                control: data.c.ok_or_else(|| missing_field("c"))?,
                target1: data.t1.ok_or_else(|| missing_field("t1"))?,
                target2: data.t2.ok_or_else(|| missing_field("t2"))?,
            }),
            "ccx" => Ok(CCX {
                control1: data.c1.ok_or_else(|| missing_field("c1"))?,
                control2: data.c2.ok_or_else(|| missing_field("c2"))?,
                target: data.t.ok_or_else(|| missing_field("t"))?,
            }),
            "ccz" => Ok(CCZ {
                qubit1: data.q1.ok_or_else(|| missing_field("q1"))?,
                qubit2: data.q2.ok_or_else(|| missing_field("q2"))?,
                qubit3: data.q3.ok_or_else(|| missing_field("q3"))?,
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

    to_vec(&datas).map_err(|error| SerializeError::SerializationFailure {
        format: ConversionFormat::MessagePack,
        message: error.to_string(),
    })
}
