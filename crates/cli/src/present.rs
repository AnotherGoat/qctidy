use std::io::{self, Write};
use std::process::ExitCode;
use std::sync::Arc;

use anstream::{AutoStream, ColorChoice};
use qctidy::Circuit;
use qctidy_converter::ConverterAdapter;
use qctidy_facade::{ParseRequest, PresentationRequest, parse, present};
use qctidy_presenter::GraphvizPresenter;

use crate::arguments::PresentArguments;
use crate::error::CliError;
use crate::input::Input;
use crate::output;
use crate::python;

/// Run the `present` command.
pub(crate) fn run(arguments: &PresentArguments, color: ColorChoice) -> ExitCode {
    let mut stderr = AutoStream::new(io::stderr(), color);
    let input = Input::resolve_one(arguments.input.as_deref());

    let failed = match render_input(&input, arguments, &mut stderr) {
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

fn render_input(
    input: &Input,
    arguments: &PresentArguments,
    stderr: &mut dyn Write,
) -> Result<bool, CliError> {
    let bytes = input.read().map_err(|error| CliError::Read {
        source_name: input.name().to_owned(),
        error,
    })?;

    if input.is_python() {
        return render_python(&bytes, input, arguments, stderr);
    }

    if arguments.circuit.is_some() {
        return Err(CliError::CircuitSelectorNotPython);
    }

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

    let output_bytes = render_circuit(parsed.circuit().as_ref(), arguments)?;
    output::write_bytes(arguments.output.as_deref(), &output_bytes)?;

    Ok(false)
}

fn render_python(
    bytes: &[u8],
    input: &Input,
    arguments: &PresentArguments,
    stderr: &mut dyn Write,
) -> Result<bool, CliError> {
    let source = String::from_utf8_lossy(bytes).into_owned();
    let outcomes = python::extract_circuits(&source, input.name(), arguments.circuit.as_deref())?;

    let mut failed = false;
    let mut outputs = Vec::new();

    for outcome in outcomes {
        match outcome {
            Ok(info) => {
                let output_bytes = render_circuit(&info.circuit, arguments)?;
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

fn render_circuit(circuit: &Circuit, arguments: &PresentArguments) -> Result<Vec<u8>, CliError> {
    let presentation_request = PresentationRequest::new(
        Arc::new(circuit.clone()),
        arguments.format.into(),
        arguments.dpi,
    );
    let response = present(&presentation_request, &GraphvizPresenter)
        .map_err(|error| CliError::Render { error })?;

    Ok(response.bytes().to_vec())
}
