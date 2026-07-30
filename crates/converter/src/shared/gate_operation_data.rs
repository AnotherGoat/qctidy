use qctidy::{GateOperation, GateType};
use qctidy_ports::ParseError;

pub(crate) struct GateOperationData {
    pub gate: GateType,
    pub qubit: Option<usize>,
    pub qubit1: Option<usize>,
    pub qubit2: Option<usize>,
    pub qubit3: Option<usize>,
    pub control: Option<usize>,
    pub control1: Option<usize>,
    pub control2: Option<usize>,
    pub target: Option<usize>,
    pub target1: Option<usize>,
    pub target2: Option<usize>,
    pub theta: Option<f64>,
    pub phi: Option<f64>,
    pub lambda: Option<f64>,
    pub bit: Option<usize>,
}

impl GateOperationData {
    pub(crate) const fn new(gate: GateType) -> Self {
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
            theta: None,
            phi: None,
            lambda: None,
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

    const fn theta(mut self, theta: f64) -> Self {
        self.theta = Some(theta);
        self
    }

    const fn phi(mut self, phi: f64) -> Self {
        self.phi = Some(phi);
        self
    }

    const fn lambda(mut self, lambda: f64) -> Self {
        self.lambda = Some(lambda);
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

        let gate = operation.r#type();

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
            P { theta, qubit } | RX { theta, qubit } | RY { theta, qubit } => {
                Self::new(gate).qubit(qubit).theta(theta)
            }
            RZ { phi, qubit } => Self::new(gate).qubit(qubit).phi(phi),
            U {
                theta,
                phi,
                lambda,
                qubit,
            } => Self::new(gate)
                .qubit(qubit)
                .theta(theta)
                .phi(phi)
                .lambda(lambda),
            Measure { qubit, bit } => Self::new(gate).qubit(qubit).bit(bit),
            Swap { qubit1, qubit2 } | CZ { qubit1, qubit2 } => {
                Self::new(gate).qubit1(qubit1).qubit2(qubit2)
            }
            CH { control, target } | CX { control, target } | CY { control, target } => {
                Self::new(gate).control(control).target(target)
            }
            CP {
                theta,
                qubit1,
                qubit2,
            } => Self::new(gate).qubit1(qubit1).qubit2(qubit2).theta(theta),
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

        let gate = data.gate;
        let missing_field = |field: &str| ParseError::MissingRequiredField {
            field: field.to_owned(),
            gate: gate.to_string(),
        };

        match gate {
            GateType::ID => Ok(ID {
                qubit: data.qubit.ok_or_else(|| missing_field("qubit"))?,
            }),
            GateType::H => Ok(H {
                qubit: data.qubit.ok_or_else(|| missing_field("qubit"))?,
            }),
            GateType::X => Ok(X {
                qubit: data.qubit.ok_or_else(|| missing_field("qubit"))?,
            }),
            GateType::Y => Ok(Y {
                qubit: data.qubit.ok_or_else(|| missing_field("qubit"))?,
            }),
            GateType::Z => Ok(Z {
                qubit: data.qubit.ok_or_else(|| missing_field("qubit"))?,
            }),
            GateType::P => Ok(P {
                qubit: data.qubit.ok_or_else(|| missing_field("qubit"))?,
                theta: data.theta.ok_or_else(|| missing_field("theta"))?,
            }),
            GateType::RX => Ok(RX {
                qubit: data.qubit.ok_or_else(|| missing_field("qubit"))?,
                theta: data.theta.ok_or_else(|| missing_field("theta"))?,
            }),
            GateType::RY => Ok(RY {
                qubit: data.qubit.ok_or_else(|| missing_field("qubit"))?,
                theta: data.theta.ok_or_else(|| missing_field("theta"))?,
            }),
            GateType::RZ => Ok(RZ {
                qubit: data.qubit.ok_or_else(|| missing_field("qubit"))?,
                phi: data.phi.ok_or_else(|| missing_field("phi"))?,
            }),
            GateType::S => Ok(S {
                qubit: data.qubit.ok_or_else(|| missing_field("qubit"))?,
            }),
            GateType::SDG => Ok(SDG {
                qubit: data.qubit.ok_or_else(|| missing_field("qubit"))?,
            }),
            GateType::SX => Ok(SX {
                qubit: data.qubit.ok_or_else(|| missing_field("qubit"))?,
            }),
            GateType::SY => Ok(SY {
                qubit: data.qubit.ok_or_else(|| missing_field("qubit"))?,
            }),
            GateType::T => Ok(T {
                qubit: data.qubit.ok_or_else(|| missing_field("qubit"))?,
            }),
            GateType::TDG => Ok(TDG {
                qubit: data.qubit.ok_or_else(|| missing_field("qubit"))?,
            }),
            GateType::U => Ok(U {
                qubit: data.qubit.ok_or_else(|| missing_field("qubit"))?,
                theta: data.theta.ok_or_else(|| missing_field("theta"))?,
                phi: data.phi.ok_or_else(|| missing_field("phi"))?,
                lambda: data.lambda.ok_or_else(|| missing_field("lambda"))?,
            }),
            GateType::Measure => Ok(Measure {
                qubit: data.qubit.ok_or_else(|| missing_field("qubit"))?,
                bit: data.bit.ok_or_else(|| missing_field("bit"))?,
            }),
            GateType::Swap => Ok(Swap {
                qubit1: data.qubit1.ok_or_else(|| missing_field("qubit1"))?,
                qubit2: data.qubit2.ok_or_else(|| missing_field("qubit2"))?,
            }),
            GateType::CH => Ok(CH {
                control: data.control.ok_or_else(|| missing_field("control"))?,
                target: data.target.ok_or_else(|| missing_field("target"))?,
            }),
            GateType::CX => Ok(CX {
                control: data.control.ok_or_else(|| missing_field("control"))?,
                target: data.target.ok_or_else(|| missing_field("target"))?,
            }),
            GateType::CY => Ok(CY {
                control: data.control.ok_or_else(|| missing_field("control"))?,
                target: data.target.ok_or_else(|| missing_field("target"))?,
            }),
            GateType::CZ => Ok(CZ {
                qubit1: data.qubit1.ok_or_else(|| missing_field("qubit1"))?,
                qubit2: data.qubit2.ok_or_else(|| missing_field("qubit2"))?,
            }),
            GateType::CP => Ok(CP {
                qubit1: data.qubit1.ok_or_else(|| missing_field("qubit1"))?,
                qubit2: data.qubit2.ok_or_else(|| missing_field("qubit2"))?,
                theta: data.theta.ok_or_else(|| missing_field("theta"))?,
            }),
            GateType::CSwap => Ok(CSwap {
                control: data.control.ok_or_else(|| missing_field("control"))?,
                target1: data.target1.ok_or_else(|| missing_field("target1"))?,
                target2: data.target2.ok_or_else(|| missing_field("target2"))?,
            }),
            GateType::CCX => Ok(CCX {
                control1: data.control1.ok_or_else(|| missing_field("control1"))?,
                control2: data.control2.ok_or_else(|| missing_field("control2"))?,
                target: data.target.ok_or_else(|| missing_field("target"))?,
            }),
            GateType::CCZ => Ok(CCZ {
                qubit1: data.qubit1.ok_or_else(|| missing_field("qubit1"))?,
                qubit2: data.qubit2.ok_or_else(|| missing_field("qubit2"))?,
                qubit3: data.qubit3.ok_or_else(|| missing_field("qubit3"))?,
            }),
        }
    }
}
