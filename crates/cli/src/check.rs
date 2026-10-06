use std::io;
use std::process::ExitCode;
use std::sync::Arc;

use anstream::{AutoStream, ColorChoice};
use qctidy::Graph;
use qctidy_converter::ConverterAdapter;
use qctidy_facade::{CheckRequest, ParseRequest, check, parse};
use qctidy_ports::ConversionFormat;

use crate::arguments::{CheckArguments, OutputFormat};
use crate::error::CliError;
use crate::input::Input;
use crate::output::{self, CircuitReport, JsonDiagnostic, Stats};
use crate::progress::Progress;

/// Run the `check` command.
pub(crate) fn run(arguments: &CheckArguments, color: ColorChoice) -> ExitCode {
    let inputs: Vec<_> = Input::resolve(&arguments.input)
        .into_iter()
        .map(|input| input.named(arguments.input_name.as_deref()))
        .collect();
    let snippets = arguments.verbose && arguments.output_format == OutputFormat::Human;
    let mut reporter = Reporter::new(arguments.output_format, color, arguments.quiet);
    let mut stats = Stats::default();

    for input in &inputs {
        let progress = Progress::start(format!("Checking {}", input.name()), color);

        match analyze(input, arguments.input_format, snippets) {
            Ok(report) => {
                drop(progress);

                stats.circuits += 1;
                stats.detections += report.diagnostics.len();
                reporter.report(&report);
            }
            Err(error) => {
                drop(progress);

                stats.errors += 1;
                reporter.error(&error);
            }
        }
    }

    reporter.finish(&stats);

    exit_code(&stats, arguments.no_fail)
}

fn analyze(
    input: &Input,
    override_format: Option<ConversionFormat>,
    snippets: bool,
) -> Result<CircuitReport, CliError> {
    let bytes = input.read().map_err(|error| CliError::Read {
        source_name: input.name().to_owned(),
        error,
    })?;

    let format =
        input
            .format(override_format, &bytes)
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

    let circuit = parsed.circuit();
    let check_request = CheckRequest::new(Arc::clone(&circuit));
    let response = check(&check_request).map_err(|error| CliError::Check {
        source_name: input.name().to_owned(),
        error,
    })?;

    let snippets = if snippets {
        let graph = Graph::from(circuit.as_ref());

        response
            .diagnostics()
            .iter()
            .map(|detection| output::detection_snippet(&graph, detection.positions()))
            .collect()
    } else {
        Vec::new()
    };

    Ok(CircuitReport {
        name: input.name().to_owned(),
        format,
        qubit_count: response.qubit_count(),
        time_step_count: response.time_step_count(),
        gate_count: response.gate_count(),
        diagnostics: response.diagnostics().clone(),
        snippets,
    })
}

fn exit_code(stats: &Stats, no_fail: bool) -> ExitCode {
    if stats.errors > 0 {
        ExitCode::from(2)
    } else if stats.detections > 0 && !no_fail {
        ExitCode::from(1)
    } else {
        ExitCode::SUCCESS
    }
}

struct Reporter {
    output_format: OutputFormat,
    quiet: bool,
    stdout: AutoStream<io::Stdout>,
    stderr: AutoStream<io::Stderr>,
    diagnostics: Vec<JsonDiagnostic>,
}

impl Reporter {
    fn new(output_format: OutputFormat, color: ColorChoice, quiet: bool) -> Self {
        Self {
            output_format,
            quiet,
            stdout: AutoStream::new(io::stdout(), color),
            stderr: AutoStream::new(io::stderr(), color),
            diagnostics: Vec::new(),
        }
    }

    fn report(&mut self, report: &CircuitReport) {
        match self.output_format {
            OutputFormat::Human => output::human_report(&mut self.stdout, report, self.quiet),
            OutputFormat::Json => self.diagnostics.extend(
                report
                    .diagnostics
                    .iter()
                    .map(|detection| JsonDiagnostic::new(&report.name, detection)),
            ),
        }
    }

    fn error(&mut self, error: &CliError) {
        output::human_error(&mut self.stderr, error);
    }

    fn finish(&mut self, stats: &Stats) {
        match self.output_format {
            OutputFormat::Human => {
                if !self.quiet {
                    output::human_summary(&mut self.stdout, stats);
                }
            }
            OutputFormat::Json => output::json_report(&mut self.stdout, &self.diagnostics),
        }
    }
}
