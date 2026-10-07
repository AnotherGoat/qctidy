use std::fs::File;
use std::io;
use std::path::Path;
use std::process::ExitCode;

use anstream::{AutoStream, ColorChoice};
use qctidy::Graph;
use qctidy_converter::ConverterAdapter;
use qctidy_facade::{ParseRequest, Session, parse};
use qctidy_ports::ConversionFormat;

use crate::arguments::{CheckArguments, OutputFormat};
use crate::error::CliError;
use crate::input::{Input, format_name};
use crate::output::{self, CircuitReport, JsonDiagnostic, Stats};
use crate::progress::Progress;
use crate::python;

/// Run the `check` command.
pub(crate) fn run(arguments: &CheckArguments, color: ColorChoice) -> ExitCode {
    let inputs: Vec<_> = Input::resolve(&arguments.input)
        .into_iter()
        .map(|input| input.named(arguments.input_name.as_deref()))
        .collect();
    let snippets = arguments.verbose && arguments.output_format == OutputFormat::Human;
    let mut reporter = match Reporter::new(
        arguments.output_format,
        color,
        arguments.quiet,
        arguments.output.as_deref(),
    ) {
        Ok(reporter) => reporter,
        Err(error) => {
            let mut stderr = AutoStream::new(io::stderr(), color);
            output::human_error(&mut stderr, &error);
            return ExitCode::from(2);
        }
    };
    let mut stats = Stats::default();
    let session = Session::default();

    for input in &inputs {
        let progress = Progress::start(format!("Checking {}", input.name()), color);

        if input.is_python() {
            drop(progress);
            report_python(
                input,
                snippets,
                &session,
                arguments.circuit.as_deref(),
                &mut reporter,
                &mut stats,
            );
            continue;
        }

        if arguments.circuit.is_some() {
            drop(progress);
            stats.errors += 1;
            reporter.error(&CliError::CircuitSelectorNotPython);
            continue;
        }

        match analyze(input, arguments.input_format, snippets, &session) {
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
    session: &Session,
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
    let response = session.check(circuit.as_ref());

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
        format: format_name(format),
        qubit_count: response.qubit_count(),
        time_step_count: response.time_step_count(),
        gate_count: response.gate_count(),
        diagnostics: response.diagnostics().clone(),
        snippets,
        source_map: None,
    })
}

fn report_python(
    input: &Input,
    snippets: bool,
    session: &Session,
    selector: Option<&str>,
    reporter: &mut Reporter,
    stats: &mut Stats,
) {
    let bytes = match input.read() {
        Ok(bytes) => bytes,
        Err(error) => {
            stats.errors += 1;
            reporter.error(&CliError::Read {
                source_name: input.name().to_owned(),
                error,
            });
            return;
        }
    };

    let source = String::from_utf8_lossy(&bytes).into_owned();
    let outcomes = match python::extract_circuits(&source, input.name(), selector) {
        Ok(outcomes) => outcomes,
        Err(error) => {
            stats.errors += 1;
            reporter.error(&error);
            return;
        }
    };

    for outcome in outcomes {
        match outcome {
            Ok(info) => {
                let check = session.check(&info.circuit);
                let circuit_snippets = if snippets {
                    let graph = Graph::from(&info.circuit);

                    check
                        .diagnostics()
                        .iter()
                        .map(|detection| output::detection_snippet(&graph, detection.positions()))
                        .collect()
                } else {
                    Vec::new()
                };

                let report = CircuitReport {
                    name: info.name,
                    format: "python",
                    qubit_count: check.qubit_count(),
                    time_step_count: check.time_step_count(),
                    gate_count: check.gate_count(),
                    diagnostics: check.diagnostics().clone(),
                    snippets: circuit_snippets,
                    source_map: Some(info.source_map),
                };

                stats.circuits += 1;
                stats.detections += report.diagnostics.len();
                reporter.report(&report);
            }
            Err(error) => {
                stats.errors += 1;
                reporter.error(&error);
            }
        }
    }
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
    stdout: Box<dyn io::Write>,
    stderr: AutoStream<io::Stderr>,
    diagnostics: Vec<JsonDiagnostic>,
}

impl Reporter {
    fn new(
        output_format: OutputFormat,
        color: ColorChoice,
        quiet: bool,
        output: Option<&Path>,
    ) -> Result<Self, CliError> {
        let stdout: Box<dyn io::Write> = match output {
            Some(path) => {
                let file = File::create(path).map_err(|error| CliError::Write {
                    target_name: path.display().to_string(),
                    error,
                })?;

                Box::new(AutoStream::new(file, ColorChoice::Never))
            }
            None => Box::new(AutoStream::new(io::stdout(), color)),
        };

        Ok(Self {
            output_format,
            quiet,
            stdout,
            stderr: AutoStream::new(io::stderr(), color),
            diagnostics: Vec::new(),
        })
    }

    fn report(&mut self, report: &CircuitReport) {
        match self.output_format {
            OutputFormat::Human => output::human_report(&mut self.stdout, report, self.quiet),
            OutputFormat::Json => {
                self.diagnostics
                    .extend(report.diagnostics.iter().map(|detection| {
                        JsonDiagnostic::new(&report.name, detection, report.source_map.as_deref())
                    }));
            }
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
