use std::str;

use qsimplify::Circuit;
use qsimplify_ports::{ConversionFormat, ParseError, SerializeError};

use quick_xml::events::attributes::Attributes;
use quick_xml::events::{BytesDecl, BytesEnd, BytesStart, Event};
use quick_xml::{Reader, Writer};

use crate::CURRENT_FORMAT_VERSION;
use crate::shared::{circuit_data::CircuitData, gate_operation_data::GateOperationData};

pub fn parse(input: &[u8]) -> Result<Circuit, ParseError> {
    let input_string = str::from_utf8(input).map_err(|error| ParseError::InvalidInput {
        format: ConversionFormat::Xml,
        message: error.to_string(),
    })?;

    let mut reader = Reader::from_str(input_string);

    reader.config_mut().trim_text(true);

    let mut buffer = Vec::new();

    let mut version = None;
    let mut qubit_count = None;

    let mut operations = Vec::new();

    loop {
        match reader.read_event_into(&mut buffer) {
            Ok(Event::Start(event)) if event.name().as_ref() == b"circuit" => {
                parse_circuit_attributes(event.attributes(), &mut version, &mut qubit_count)?;
            }

            Ok(Event::Empty(event)) if event.name().as_ref() == b"gate" => {
                let operation = parse_gate_attributes(event.attributes())?;

                operations.push(operation);
            }

            Ok(Event::Eof) => break,

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

    let circuit_data = CircuitData {
        version,
        qubit_count: qubit_count.ok_or_else(|| ParseError::MissingRequiredField {
            format: ConversionFormat::Xml,
            field: "qubit_count".to_owned(),
            gate: "circuit".to_owned(),
        })?,
        operations,
    };

    Circuit::try_from(circuit_data)
}

fn parse_circuit_attributes(
    attributes: Attributes,
    version: &mut Option<u16>,
    qubit_count: &mut Option<usize>,
) -> Result<(), ParseError> {
    for attribute in attributes {
        let attribute = attribute.map_err(|error| ParseError::InvalidInput {
            format: ConversionFormat::Xml,
            message: error.to_string(),
        })?;

        let key = String::from_utf8_lossy(attribute.key.as_ref());

        let value = String::from_utf8_lossy(attribute.value.as_ref());

        match key.as_ref() {
            "version" => {
                *version = value.parse().ok();
            }

            "qubit_count" => {
                *qubit_count = value.parse().ok();
            }

            _ => {
                return Err(ParseError::UnknownField {
                    field: key.to_string(),
                    gate: "circuit".to_owned(),
                });
            }
        }
    }

    Ok(())
}

fn parse_gate_attributes(attributes: Attributes) -> Result<GateOperationData, ParseError> {
    let mut data = GateOperationData::new(String::new());

    for attribute in attributes {
        let attribute = attribute.map_err(|error| ParseError::InvalidInput {
            format: ConversionFormat::Xml,
            message: error.to_string(),
        })?;

        let key = String::from_utf8_lossy(attribute.key.as_ref());

        let value = String::from_utf8_lossy(attribute.value.as_ref());

        match key.as_ref() {
            "type" => data.gate = value.to_string(),

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
                    field: key.to_string(),
                    gate: data.gate.clone(),
                });
            }
        }
    }

    Ok(data)
}

pub fn serialize(
    circuit: &Circuit,
    prettify: bool,
    indentation: usize,
) -> Result<Vec<u8>, SerializeError> {
    let circuit_data = CircuitData::from(circuit);

    let mut buffer = Vec::new();

    let mut writer = if prettify {
        Writer::new_with_indent(&mut buffer, b' ', indentation.max(1))
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

    let mut circuit_element = BytesStart::new("circuit");

    let version = circuit_data.version.unwrap_or(CURRENT_FORMAT_VERSION);

    let version_string = version.to_string();
    let qubit_count_string = circuit_data.qubit_count.to_string();

    circuit_element.push_attribute(("version", version_string.as_str()));
    circuit_element.push_attribute(("qubit_count", qubit_count_string.as_str()));

    writer
        .write_event(Event::Start(circuit_element))
        .map_err(write_error)?;

    for operation in &circuit_data.operations {
        write_gate(&mut writer, operation).map_err(write_error)?;
    }

    writer
        .write_event(Event::End(BytesEnd::new("circuit")))
        .map_err(write_error)?;

    Ok(buffer)
}

fn write_gate(
    writer: &mut Writer<&mut Vec<u8>>,
    operation: &GateOperationData,
) -> Result<(), quick_xml::Error> {
    let mut element = BytesStart::new("gate");

    element.push_attribute(("type", operation.gate.as_str()));

    push_optional_attribute(&mut element, "qubit", operation.qubit);
    push_optional_attribute(&mut element, "qubit1", operation.qubit1);
    push_optional_attribute(&mut element, "qubit2", operation.qubit2);
    push_optional_attribute(&mut element, "qubit3", operation.qubit3);

    push_optional_attribute(&mut element, "control", operation.control);
    push_optional_attribute(&mut element, "control1", operation.control1);
    push_optional_attribute(&mut element, "control2", operation.control2);

    push_optional_attribute(&mut element, "target", operation.target);
    push_optional_attribute(&mut element, "target1", operation.target1);
    push_optional_attribute(&mut element, "target2", operation.target2);

    push_optional_attribute(&mut element, "bit", operation.bit);

    if let Some(angle) = operation.angle {
        let angle_string = angle.to_string();
        element.push_attribute(("angle", angle_string.as_str()));
    }

    writer.write_event(Event::Empty(element))
}

fn push_optional_attribute(element: &mut BytesStart, key: &'static str, value: Option<usize>) {
    if let Some(value) = value {
        let value_string = value.to_string();
        element.push_attribute((key, value_string.as_str()));
    }
}
