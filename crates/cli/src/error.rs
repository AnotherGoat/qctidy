use std::io;

use qctidy_ports::{ConversionFormat, ParseError, PresentationError, SerializeError};
use thiserror::Error;

/// An error that prevents a command from completing.
#[derive(Debug, Error)]
pub(crate) enum CliError {
    #[error("failed to read '{source_name}': {error}")]
    Read {
        source_name: String,
        error: io::Error,
    },
    #[error("failed to write '{target_name}': {error}")]
    Write {
        target_name: String,
        error: io::Error,
    },
    #[error("could not determine the input format for '{source_name}', pass --input-format")]
    UnknownInputFormat { source_name: String },
    #[error("could not determine the output format for '{target_name}', pass --output-format")]
    UnknownOutputFormat { target_name: String },
    #[error("the format '{format}' is not supported by this build")]
    UnsupportedFormat { format: ConversionFormat },
    #[error("failed to parse '{source_name}': {error}")]
    Parse {
        source_name: String,
        error: ParseError,
    },
    #[error("'{source_name}' cannot be checked: {message}")]
    UncheckableCircuit {
        source_name: String,
        message: String,
    },
    #[error("circuit '{selector}' not found in '{source_name}'")]
    UnknownCircuit {
        source_name: String,
        selector: String,
    },
    #[error(
        "circuit '{selector}' matches multiple circuits in '{source_name}'; add ':line' to disambiguate"
    )]
    AmbiguousCircuit {
        source_name: String,
        selector: String,
    },
    #[error(
        "multiple circuits require --output to write numbered files, or --circuit to select one"
    )]
    MultipleOutputs,
    #[error("--circuit only applies to Python (.py) inputs")]
    CircuitSelectorNotPython,
    #[error("unknown rule or category code '{selector}'")]
    UnknownRuleSelector { selector: String },
    #[error("failed to serialize the circuit: {error}")]
    Serialize { error: SerializeError },
    #[error("failed to serialize the analysis: {error}")]
    SerializeJson { error: serde_json::Error },
    #[error("failed to render the circuit: {error}")]
    Render { error: PresentationError },
}
