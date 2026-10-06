use std::io;

use qctidy_ports::{
    CheckError, ConversionFormat, DisplayError, ParseError, PresentationError, SerializeError,
};
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
    #[error("failed to serialize the circuit: {error}")]
    Serialize { error: SerializeError },
    #[error("failed to display the circuit: {error}")]
    Display { error: DisplayError },
    #[error("failed to render the circuit: {error}")]
    Render { error: PresentationError },
    #[error("failed to check '{source_name}': {error}")]
    Check {
        source_name: String,
        error: CheckError,
    },
}
