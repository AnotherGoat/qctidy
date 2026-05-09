use std::str;

use qsimplify::GateOperation;
use qsimplify_ports::{ConversionFormat, ParseError, SerializeError};
use quick_xml::events::Event;
use quick_xml::events::attributes::Attributes;
use quick_xml::events::{BytesDecl, BytesEnd, BytesStart};
use quick_xml::{Reader, Writer};

#[derive(Debug)]
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
    fn from_attributes(attributes: Attributes) -> Result<Self, ParseError> {
        let mut data = Self {
            gate: String::new(),
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
        };

        for attribute in attributes {
            let xml_attribute = attribute.map_err(|error| ParseError::InvalidInput {
                format: ConversionFormat::Xml,
                message: error.to_string(),
            })?;
            let key = String::from_utf8_lossy(xml_attribute.key.as_ref()).to_string();
            let value = String::from_utf8_lossy(xml_attribute.value.as_ref()).to_string();

            match key.as_str() {
                "type" => data.gate = value,
                "qubit" => data.qubit = value.parse().ok(),
                "qubit1" => data.qubit1 = value.parse().ok(),
                "qubit2" => data.qubit2 = value.parse().ok(),
                "qubit3" => data.qubit3 = value.parse().ok(),
                "control" => data.control = value.parse().ok(),
                "control1" => data.control1 = value.parse().ok(),
                "control2" => data.control2 = value.parse().ok(),
                "target" => data.target = value.parse().ok(),
                "target1" => data.target1 = value.parse().ok(),
                "target2" => data.target2 = value.parse().ok(),
                "angle" => data.angle = value.parse().ok(),
                "bit" => data.bit = value.parse().ok(),
                _ => {
                    return Err(ParseError::UnknownField {
                        field: key,
                        gate: data.gate,
                    });
                }
            }
        }

        Ok(data)
    }
}

impl TryFrom<GateOperationData> for GateOperation {
    type Error = ParseError;

    fn try_from(data: GateOperationData) -> Result<Self, Self::Error> {
        use GateOperation::*;

        let gate = data.gate.as_str();
        let missing_field = |field: &str| ParseError::MissingRequiredField {
            format: ConversionFormat::Xml,
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

pub(crate) fn serialize(
    operations: &[GateOperation],
    prettify: bool,
    indentation: usize,
) -> Result<Vec<u8>, SerializeError> {
    let mut buffer = Vec::new();

    let mut writer = if prettify {
        let indent = indentation.max(1);
        Writer::new_with_indent(&mut buffer, b' ', indent)
    } else {
        Writer::new(&mut buffer)
    };

    let write_error = |error: quick_xml::Error| SerializeError::SerializationFailure {
        format: ConversionFormat::Xml,
        message: error.to_string(),
    };

    writer
        .write_event(Event::Decl(BytesDecl::new("1.0", Some("utf-8"), None)))
        .map_err(write_error)?;

    writer
        .write_event(Event::Start(BytesStart::new("gates")))
        .map_err(write_error)?;

    for operation in operations {
        write_gate(&mut writer, operation).map_err(write_error)?;
    }

    writer
        .write_event(Event::End(BytesEnd::new("gates")))
        .map_err(write_error)?;

    Ok(buffer)
}

fn write_gate(
    writer: &mut Writer<&mut Vec<u8>>,
    operation: &GateOperation,
) -> Result<(), quick_xml::Error> {
    use GateOperation::*;

    let gate_type = operation.r#type().to_string();
    let mut element = BytesStart::new("gate");
    element.push_attribute(("type", gate_type.as_str()));

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
        | TDG { qubit } => {
            element.push_attribute(("qubit", qubit.to_string().as_str()));
        }
        P { angle, qubit } | RX { angle, qubit } | RY { angle, qubit } | RZ { angle, qubit } => {
            element.push_attribute(("qubit", qubit.to_string().as_str()));
            element.push_attribute(("angle", angle.to_string().as_str()));
        }
        Measure { qubit, bit } => {
            element.push_attribute(("qubit", qubit.to_string().as_str()));
            element.push_attribute(("bit", bit.to_string().as_str()));
        }
        Swap { qubit1, qubit2 } | CZ { qubit1, qubit2 } => {
            element.push_attribute(("qubit1", qubit1.to_string().as_str()));
            element.push_attribute(("qubit2", qubit2.to_string().as_str()));
        }
        CH { control, target } | CX { control, target } | CY { control, target } => {
            element.push_attribute(("control", control.to_string().as_str()));
            element.push_attribute(("target", target.to_string().as_str()));
        }
        CP {
            angle,
            qubit1,
            qubit2,
        } => {
            element.push_attribute(("qubit1", qubit1.to_string().as_str()));
            element.push_attribute(("qubit2", qubit2.to_string().as_str()));
            element.push_attribute(("angle", angle.to_string().as_str()));
        }
        CSwap {
            control,
            target1,
            target2,
        } => {
            element.push_attribute(("control", control.to_string().as_str()));
            element.push_attribute(("target1", target1.to_string().as_str()));
            element.push_attribute(("target2", target2.to_string().as_str()));
        }
        CCX {
            control1,
            control2,
            target,
        } => {
            element.push_attribute(("control1", control1.to_string().as_str()));
            element.push_attribute(("control2", control2.to_string().as_str()));
            element.push_attribute(("target", target.to_string().as_str()));
        }
        CCZ {
            qubit1,
            qubit2,
            qubit3,
        } => {
            element.push_attribute(("qubit1", qubit1.to_string().as_str()));
            element.push_attribute(("qubit2", qubit2.to_string().as_str()));
            element.push_attribute(("qubit3", qubit3.to_string().as_str()));
        }
    }

    writer.write_event(Event::Empty(element))
}

pub(crate) fn parse(input: &[u8]) -> Result<Vec<GateOperation>, ParseError> {
    let input_str = str::from_utf8(input).map_err(|error| ParseError::InvalidInput {
        format: ConversionFormat::Xml,
        message: error.to_string(),
    })?;

    let mut reader = Reader::from_str(input_str);
    reader.config_mut().trim_text(true);

    let mut operations = Vec::new();
    let mut buffer = Vec::new();

    loop {
        match reader.read_event_into(&mut buffer) {
            Ok(Event::Empty(error) | Event::Start(error)) if error.name().as_ref() == b"gate" => {
                let data = GateOperationData::from_attributes(error.attributes())?;
                operations.push(GateOperation::try_from(data)?);
            }
            Ok(Event::Eof) => break,
            Ok(Event::Start(error)) if error.name().as_ref() == b"gates" => {}
            Ok(Event::End(error)) if error.name().as_ref() == b"gates" => {}
            Err(error) => {
                return Err(ParseError::InvalidInput {
                    format: ConversionFormat::Xml,
                    message: error.to_string(),
                });
            }
            _ => {}
        }
        buffer.clear();
    }

    Ok(operations)
}
