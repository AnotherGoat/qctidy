use std::path::PathBuf;

use anstream::ColorChoice;
use clap::{Args, Parser, Subcommand, ValueEnum};
use qctidy_facade::DisplayFormat;
use qctidy_ports::{ConversionFormat, PresentationFormat};

#[derive(Debug, Parser)]
#[command(name = "qctidy", version)]
#[expect(clippy::field_scoped_visibility_modifiers)]
pub(crate) struct Cli {
    /// When to use colors in the output.
    #[arg(long, value_enum, default_value_t = ColorMode::Auto, global = true)]
    pub(crate) color: ColorMode,

    #[command(subcommand)]
    pub(crate) command: Command,
}

#[derive(Debug, Subcommand)]
pub(crate) enum Command {
    /// Analyze a Python file and dump its circuits as JSON.
    Ast(AstArguments),
    /// Detect fixable patterns in circuits without modifying them.
    Check(CheckArguments),
    /// Convert a circuit between formats.
    Convert(ConvertArguments),
    /// Display a circuit as text.
    Display(DisplayArguments),
    /// Render a circuit as a graphviz graph.
    Present(PresentArguments),
    /// List all the available fix rules.
    Rules,
}

#[derive(Debug, Args)]
#[expect(clippy::field_scoped_visibility_modifiers)]
pub(crate) struct AstArguments {
    /// Python file to analyze (reads from standard input when omitted).
    #[arg(short, long, value_name = "FILE")]
    pub(crate) input: Option<PathBuf>,

    /// File to write the analysis to (writes to standard output when omitted).
    #[arg(short, long, value_name = "FILE")]
    pub(crate) output: Option<PathBuf>,
}

#[derive(Debug, Args)]
#[expect(clippy::field_scoped_visibility_modifiers)]
pub(crate) struct CheckArguments {
    /// Circuit files to check (reads from standard input when omitted).
    #[arg(short, long, value_name = "FILE")]
    pub(crate) input: Vec<PathBuf>,

    /// Input format, overriding the guessed format.
    ///
    /// Accepted values: json, xml, msgpack, cbor.
    #[arg(long, value_name = "FORMAT", value_parser = parse_conversion_format)]
    pub(crate) input_format: Option<ConversionFormat>,

    /// Name reported for standard input, instead of `<stdin>`.
    #[arg(long, value_name = "NAME")]
    pub(crate) input_name: Option<String>,

    /// Select a single circuit from a Python file, by variable name or `name:line`.
    #[arg(long, value_name = "NAME")]
    pub(crate) circuit: Option<String>,

    /// Only report diagnostics for the given rule or category codes.
    ///
    /// Accepts a comma-separated list, such as `R` or `R001,R002`.
    #[arg(long, value_name = "CODES", value_delimiter = ',')]
    pub(crate) select: Vec<String>,

    /// Do not report diagnostics for the given rule or category codes.
    ///
    /// Accepts a comma-separated list, such as `R` or `R001,R002`.
    #[arg(long, value_name = "CODES", value_delimiter = ',')]
    pub(crate) ignore: Vec<String>,

    /// File to write the report to (writes to standard output when omitted).
    #[arg(short, long, value_name = "FILE")]
    pub(crate) output: Option<PathBuf>,

    /// Format used to report the results.
    #[arg(long, value_enum, default_value_t = OutputFormat::Human)]
    pub(crate) output_format: OutputFormat,

    /// Print only the diagnostics, without headers or summaries.
    #[arg(short, long)]
    pub(crate) quiet: bool,

    /// Exit with code 0 even when fixes are detected.
    #[arg(long)]
    pub(crate) no_fail: bool,

    /// Show a circuit snippet around every diagnostic.
    #[arg(short, long, conflicts_with = "quiet")]
    pub(crate) verbose: bool,
}

#[derive(Debug, Args)]
#[expect(clippy::field_scoped_visibility_modifiers)]
pub(crate) struct ConvertArguments {
    /// Circuit file to convert (reads from standard input when omitted).
    #[arg(short, long, value_name = "FILE")]
    pub(crate) input: Option<PathBuf>,

    /// File to write the converted circuit to (writes to standard output when omitted).
    #[arg(short, long, value_name = "FILE")]
    pub(crate) output: Option<PathBuf>,

    /// Input format, overriding the guessed format.
    ///
    /// Accepted values: json, xml, msgpack, cbor.
    #[arg(long, value_name = "FORMAT", value_parser = parse_conversion_format)]
    pub(crate) input_format: Option<ConversionFormat>,

    /// Output format, overriding the format guessed from the output file.
    ///
    /// Accepted values: json, xml, msgpack, cbor.
    #[arg(long, value_name = "FORMAT", value_parser = parse_conversion_format)]
    pub(crate) output_format: Option<ConversionFormat>,

    /// Pretty-print the output, for formats that support it.
    #[arg(long)]
    pub(crate) prettify: bool,

    /// Number of spaces per indentation level.
    #[arg(long, value_name = "N", requires = "prettify")]
    pub(crate) indentation: Option<usize>,

    /// Select a single circuit from a Python file, by variable name or `name:line`.
    #[arg(long, value_name = "NAME")]
    pub(crate) circuit: Option<String>,
}

#[derive(Debug, Args)]
#[expect(clippy::field_scoped_visibility_modifiers)]
pub(crate) struct DisplayArguments {
    /// Circuit file to display (reads from standard input when omitted).
    #[arg(short, long, value_name = "FILE")]
    pub(crate) input: Option<PathBuf>,

    /// Input format, overriding the guessed format.
    ///
    /// Accepted values: json, xml, msgpack, cbor.
    #[arg(long, value_name = "FORMAT", value_parser = parse_conversion_format)]
    pub(crate) input_format: Option<ConversionFormat>,

    /// How to display the circuit.
    #[arg(long, value_enum, default_value_t = DisplayMode::Grid)]
    pub(crate) format: DisplayMode,

    /// File to write the display to (writes to standard output when omitted).
    #[arg(short, long, value_name = "FILE")]
    pub(crate) output: Option<PathBuf>,

    /// Select a single circuit from a Python file, by variable name or `name:line`.
    #[arg(long, value_name = "NAME")]
    pub(crate) circuit: Option<String>,
}

#[derive(Debug, Args)]
#[expect(clippy::field_scoped_visibility_modifiers)]
pub(crate) struct PresentArguments {
    /// Circuit file to render (reads from standard input when omitted).
    #[arg(short, long, value_name = "FILE")]
    pub(crate) input: Option<PathBuf>,

    /// Input format, overriding the guessed format.
    ///
    /// Accepted values: json, xml, msgpack, cbor.
    #[arg(long, value_name = "FORMAT", value_parser = parse_conversion_format)]
    pub(crate) input_format: Option<ConversionFormat>,

    /// Image format used to render the circuit.
    #[arg(long, value_enum, default_value_t = PresentationMode::Svg)]
    pub(crate) format: PresentationMode,

    /// File to write the image to (writes to standard output when omitted).
    #[arg(short, long, value_name = "FILE")]
    pub(crate) output: Option<PathBuf>,

    /// Resolution in dots per inch.
    #[arg(long, value_name = "DPI")]
    pub(crate) dpi: Option<u32>,

    /// Select a single circuit from a Python file, by variable name or `name:line`.
    #[arg(long, value_name = "NAME")]
    pub(crate) circuit: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub(crate) enum OutputFormat {
    /// Human-readable report.
    Human,
    /// JSON report following static analysis conventions.
    Json,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub(crate) enum ColorMode {
    /// Colorize only when the output is a terminal.
    Auto,
    /// Always colorize.
    Always,
    /// Never colorize.
    Never,
}

impl From<ColorMode> for ColorChoice {
    fn from(mode: ColorMode) -> Self {
        match mode {
            ColorMode::Auto => Self::Auto,
            ColorMode::Always => Self::Always,
            ColorMode::Never => Self::Never,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub(crate) enum DisplayMode {
    /// List nodes and edges.
    Graph,
    /// Draw the circuit as a grid.
    Grid,
    /// Draw the circuit's unitary matrix.
    Matrix,
    /// List the circuit operations.
    Circuit,
}

impl From<DisplayMode> for DisplayFormat {
    fn from(mode: DisplayMode) -> Self {
        match mode {
            DisplayMode::Graph => Self::Graph,
            DisplayMode::Grid => Self::Grid,
            DisplayMode::Matrix => Self::Matrix,
            DisplayMode::Circuit => Self::Circuit,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub(crate) enum PresentationMode {
    /// Graphviz DOT source.
    Gv,
    /// Rasterized PNG image.
    Png,
    /// Vector SVG image.
    Svg,
}

impl From<PresentationMode> for PresentationFormat {
    fn from(mode: PresentationMode) -> Self {
        match mode {
            PresentationMode::Gv => Self::GraphvizGv,
            PresentationMode::Png => Self::GraphvizPng,
            PresentationMode::Svg => Self::GraphvizSvg,
        }
    }
}

fn parse_conversion_format(value: &str) -> Result<ConversionFormat, String> {
    match value.to_ascii_lowercase().as_str() {
        "json" => Ok(ConversionFormat::Json),
        "xml" => Ok(ConversionFormat::Xml),
        "msgpack" | "messagepack" | "message_pack" => Ok(ConversionFormat::MessagePack),
        "cbor" => Ok(ConversionFormat::Cbor),
        _ => Err(format!(
            "unknown input format '{value}', expected one of: json, xml, msgpack, cbor"
        )),
    }
}
