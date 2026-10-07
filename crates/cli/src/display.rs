use std::fmt::Write as _;
use std::io::{self, Write};
use std::process::ExitCode;
use std::sync::Arc;

use anstream::{AutoStream, ColorChoice};
use qctidy_converter::ConverterAdapter;
use qctidy_facade::{DisplayFormat, DisplayRequest, ParseRequest, display, parse};

use crate::arguments::DisplayArguments;
use crate::error::CliError;
use crate::input::Input;
use crate::output;
use crate::python;

/// Run the `display` command.
pub(crate) fn run(arguments: &DisplayArguments, color: ColorChoice) -> ExitCode {
    let mut stderr = AutoStream::new(io::stderr(), color);
    let input = Input::resolve_one(arguments.input.as_deref());

    let result = if input.is_python() {
        display_python(&input, arguments, &mut stderr)
    } else if arguments.circuit.is_some() {
        Err(CliError::CircuitSelectorNotPython)
    } else {
        display_serialized(&input, arguments).map(|text| (text, false))
    };

    let (text, failed) = match result {
        Ok(result) => result,
        Err(error) => {
            output::human_error(&mut stderr, &error);
            return ExitCode::from(2);
        }
    };

    let mut bytes = text.into_bytes();
    if !bytes.ends_with(b"\n") {
        bytes.push(b'\n');
    }

    if let Err(error) = output::write_bytes(arguments.output.as_deref(), &bytes) {
        output::human_error(&mut stderr, &error);
        return ExitCode::from(2);
    }

    if failed {
        ExitCode::from(2)
    } else {
        ExitCode::SUCCESS
    }
}

fn display_serialized(input: &Input, arguments: &DisplayArguments) -> Result<String, CliError> {
    let bytes = input.read().map_err(|error| CliError::Read {
        source_name: input.name().to_owned(),
        error,
    })?;

    let format = input
        .format(arguments.input_format, &bytes)
        .ok_or_else(|| CliError::UnknownInputFormat {
            source_name: input.name().to_owned(),
        })?;

    if !format.is_available() {
        return Err(CliError::UnsupportedFormat { format });
    }

    let parse_request = ParseRequest::new(bytes.into(), format);
    let parsed = parse(&parse_request, &ConverterAdapter).map_err(|error| CliError::Parse {
        source_name: input.name().to_owned(),
        error,
    })?;

    let display_request = DisplayRequest::new(
        parsed.circuit(),
        DisplayFormat::from(arguments.format),
        None,
        None,
    );

    Ok(display(&display_request).text().clone())
}

fn display_python(
    input: &Input,
    arguments: &DisplayArguments,
    stderr: &mut dyn Write,
) -> Result<(String, bool), CliError> {
    let bytes = input.read().map_err(|error| CliError::Read {
        source_name: input.name().to_owned(),
        error,
    })?;

    let source = String::from_utf8_lossy(&bytes).into_owned();
    let outcomes = python::extract_circuits(&source, input.name(), arguments.circuit.as_deref())?;

    let multiple = outcomes.iter().filter(|outcome| outcome.is_ok()).count() > 1;
    let mut failed = false;
    let mut output = String::new();

    for (index, outcome) in outcomes.into_iter().enumerate() {
        match outcome {
            Ok(info) => {
                if multiple {
                    if index > 0 {
                        output.push('\n');
                    }

                    let _header_result = writeln!(output, "circuit {}", info.name);
                }

                let display_request = DisplayRequest::new(
                    Arc::new(info.circuit),
                    DisplayFormat::from(arguments.format),
                    None,
                    None,
                );
                output.push_str(display(&display_request).text());
            }
            Err(error) => {
                failed = true;
                output::human_error(stderr, &error);
            }
        }
    }

    Ok((output, failed))
}
