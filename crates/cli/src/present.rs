use std::io;
use std::process::ExitCode;

use anstream::{AutoStream, ColorChoice};
use qctidy_converter::ConverterAdapter;
use qctidy_facade::{ParseRequest, PresentationRequest, parse, present};
use qctidy_presenter::GraphvizPresenter;

use crate::arguments::PresentArguments;
use crate::error::CliError;
use crate::input::Input;
use crate::output;

/// Run the `present` command.
pub(crate) fn run(arguments: &PresentArguments, color: ColorChoice) -> ExitCode {
    let mut stderr = AutoStream::new(io::stderr(), color);
    let input = Input::resolve_one(arguments.input.as_deref());

    let bytes = match render(&input, arguments) {
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

fn render(input: &Input, arguments: &PresentArguments) -> Result<Vec<u8>, CliError> {
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

    let presentation_request =
        PresentationRequest::new(parsed.circuit(), arguments.format.into(), arguments.dpi);
    let response = present(&presentation_request, &GraphvizPresenter)
        .map_err(|error| CliError::Render { error })?;

    Ok(response.bytes().to_vec())
}
