use std::io::{self, Write};
use std::process::ExitCode;

use anstream::{AutoStream, ColorChoice};
use qctidy_converter::ConverterAdapter;
use qctidy_facade::{DisplayFormat, DisplayRequest, ParseRequest, display, parse};

use crate::arguments::DisplayArguments;
use crate::error::CliError;
use crate::input::Input;
use crate::output;

/// Run the `display` command.
pub(crate) fn run(arguments: &DisplayArguments, color: ColorChoice) -> ExitCode {
    let mut stderr = AutoStream::new(io::stderr(), color);
    let input = Input::resolve_one(arguments.input.as_deref());

    match display_circuit(&input, arguments) {
        Ok(text) => {
            let mut stdout = AutoStream::new(io::stdout(), color);

            let _text_result = write!(stdout, "{text}");
            if !text.ends_with('\n') {
                let _newline_result = writeln!(stdout);
            }

            ExitCode::SUCCESS
        }
        Err(error) => {
            output::human_error(&mut stderr, &error);
            ExitCode::from(2)
        }
    }
}

fn display_circuit(input: &Input, arguments: &DisplayArguments) -> Result<String, CliError> {
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
    let response = display(&display_request).map_err(|error| CliError::Display { error })?;

    Ok(response.text().clone())
}
