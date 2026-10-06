use std::io;
use std::process::ExitCode;

use anstream::{AutoStream, ColorChoice};
use qctidy_converter::ConverterAdapter;
use qctidy_facade::{ParseRequest, SerializeRequest, parse, serialize};
use qctidy_ports::ConversionFormat;

use crate::arguments::ConvertArguments;
use crate::error::CliError;
use crate::input::{Input, format_from_extension};
use crate::output;

/// Run the `convert` command.
pub(crate) fn run(arguments: &ConvertArguments, color: ColorChoice) -> ExitCode {
    let mut stderr = AutoStream::new(io::stderr(), color);
    let input = Input::resolve_one(arguments.input.as_deref());

    let bytes = match convert(&input, arguments) {
        Ok(bytes) => bytes,
        Err(error) => {
            output::human_error(&mut stderr, &error);
            return ExitCode::from(2);
        }
    };

    match output::write_bytes(arguments.output.as_deref(), &bytes) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            output::human_error(&mut stderr, &error);
            ExitCode::from(2)
        }
    }
}

fn convert(input: &Input, arguments: &ConvertArguments) -> Result<Vec<u8>, CliError> {
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

    let serialize_request = SerializeRequest::new(
        parsed.circuit(),
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
