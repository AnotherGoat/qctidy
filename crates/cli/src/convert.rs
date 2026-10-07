use std::io::{self, Write};
use std::process::ExitCode;
use std::sync::Arc;

use anstream::{AutoStream, ColorChoice};
use qctidy::Circuit;
use qctidy_converter::ConverterAdapter;
use qctidy_facade::{ParseRequest, SerializeRequest, parse, serialize};
use qctidy_ports::ConversionFormat;

use crate::arguments::ConvertArguments;
use crate::error::CliError;
use crate::input::{Input, format_from_extension};
use crate::output;
use crate::python;

/// Run the `convert` command.
pub(crate) fn run(arguments: &ConvertArguments, color: ColorChoice) -> ExitCode {
    let mut stderr = AutoStream::new(io::stderr(), color);
    let input = Input::resolve_one(arguments.input.as_deref());

    let failed = match convert_input(&input, arguments, &mut stderr) {
        Ok(failed) => failed,
        Err(error) => {
            output::human_error(&mut stderr, &error);
            return ExitCode::from(2);
        }
    };

    if failed {
        ExitCode::from(2)
    } else {
        ExitCode::SUCCESS
    }
}

fn convert_input(
    input: &Input,
    arguments: &ConvertArguments,
    stderr: &mut dyn Write,
) -> Result<bool, CliError> {
    let output_format = arguments
        .output_format
        .or_else(|| arguments.output.as_deref().and_then(format_from_extension))
        .ok_or_else(|| CliError::UnknownOutputFormat {
            target_name: output_name(arguments),
        })?;

    if !output_format.is_available() {
        return Err(CliError::UnsupportedFormat {
            format: output_format,
        });
    }

    let bytes = input.read().map_err(|error| CliError::Read {
        source_name: input.name().to_owned(),
        error,
    })?;

    if input.is_python() {
        return convert_python(&bytes, input, arguments, output_format, stderr);
    }

    if arguments.circuit.is_some() {
        return Err(CliError::CircuitSelectorNotPython);
    }

    let input_format = input
        .format(arguments.input_format, &bytes)
        .ok_or_else(|| CliError::UnknownInputFormat {
            source_name: input.name().to_owned(),
        })?;

    if !input_format.is_available() {
        return Err(CliError::UnsupportedFormat {
            format: input_format,
        });
    }

    let parse_request = ParseRequest::new(bytes.into(), input_format);
    let parsed = parse(&parse_request, &ConverterAdapter).map_err(|error| CliError::Parse {
        source_name: input.name().to_owned(),
        error,
    })?;

    let output_bytes = serialize_circuit(parsed.circuit().as_ref(), output_format, arguments)?;
    output::write_bytes(arguments.output.as_deref(), &output_bytes)?;

    Ok(false)
}

fn convert_python(
    bytes: &[u8],
    input: &Input,
    arguments: &ConvertArguments,
    output_format: ConversionFormat,
    stderr: &mut dyn Write,
) -> Result<bool, CliError> {
    let source = String::from_utf8_lossy(bytes).into_owned();
    let outcomes = python::extract_circuits(&source, input.name(), arguments.circuit.as_deref())?;

    let mut failed = false;
    let mut outputs = Vec::new();

    for outcome in outcomes {
        match outcome {
            Ok(info) => {
                let output_bytes = serialize_circuit(&info.circuit, output_format, arguments)?;
                outputs.push(output_bytes);
            }
            Err(error) => {
                failed = true;
                output::human_error(stderr, &error);
            }
        }
    }

    output::write_outputs(arguments.output.as_deref(), &outputs)?;

    Ok(failed)
}

fn serialize_circuit(
    circuit: &Circuit,
    output_format: ConversionFormat,
    arguments: &ConvertArguments,
) -> Result<Vec<u8>, CliError> {
    let serialize_request = SerializeRequest::new(
        Arc::new(circuit.clone()),
        output_format,
        Some(arguments.prettify),
        arguments.indentation,
    );
    let response = serialize(&serialize_request, &ConverterAdapter)
        .map_err(|error| CliError::Serialize { error })?;

    let mut output_bytes = response.bytes().to_vec();

    if arguments.output.is_none()
        && matches!(
            output_format,
            ConversionFormat::Json | ConversionFormat::Xml
        )
        && !output_bytes.ends_with(b"\n")
    {
        output_bytes.push(b'\n');
    }

    Ok(output_bytes)
}

fn output_name(arguments: &ConvertArguments) -> String {
    arguments
        .output
        .as_ref()
        .map_or_else(|| "<stdout>".to_owned(), |path| path.display().to_string())
}
