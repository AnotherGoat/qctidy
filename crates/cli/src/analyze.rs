use std::fs::File;
use std::io::{self, Write};
use std::path::Path;
use std::process::ExitCode;
use std::sync::Arc;

use anstream::{AutoStream, ColorChoice};
use qctidy_analyzer::AnalyzerAdapter;
use qctidy_converter::ConverterAdapter;
use qctidy_facade::{AnalysisRequest, ParseRequest, analyze, parse};
use qctidy_ports::{
    AnalysisMetrics, AnalysisMode, AnalysisResult, ConversionFormat, DetailedAnalysisMetrics,
};
use serde::Serialize;

use crate::arguments::{AnalyzeArguments, OutputFormat};
use crate::error::CliError;
use crate::input::{Input, format_name};
use crate::output::{self, JsonError};
use crate::python;

/// Run the `analyze` command.
pub(crate) fn run(arguments: &AnalyzeArguments, color: ColorChoice) -> ExitCode {
    let mode = if arguments.detailed() {
        AnalysisMode::Detailed
    } else {
        AnalysisMode::Simple
    };
    let inputs: Vec<_> = Input::resolve(arguments.input())
        .into_iter()
        .map(|input| input.named(arguments.input_name().as_deref()))
        .collect();
    let mut reporter = match Reporter::new(
        arguments.output_format(),
        color,
        arguments.output().as_deref(),
    ) {
        Ok(reporter) => reporter,
        Err(error) => {
            let mut stderr = AutoStream::new(io::stderr(), color);
            output::human_error(&mut stderr, &error);
            return ExitCode::from(2);
        }
    };
    let mut circuits = 0;
    let mut errors = 0;

    for input in &inputs {
        if input.is_python() {
            analyze_python(
                input,
                arguments.circuit().as_deref(),
                mode,
                &mut reporter,
                &mut circuits,
                &mut errors,
            );
            continue;
        }

        if arguments.circuit().is_some() {
            errors += 1;
            reporter.error(&CliError::CircuitSelectorNotPython);
            continue;
        }

        match analyze_circuit(input, arguments.input_format(), mode) {
            Ok(report) => {
                circuits += 1;
                reporter.report(&report);
            }
            Err(error) => {
                errors += 1;
                reporter.error(&error);
            }
        }
    }

    reporter.finish(circuits, errors);

    if errors > 0 {
        ExitCode::from(2)
    } else {
        ExitCode::SUCCESS
    }
}

fn analyze_circuit(
    input: &Input,
    override_format: Option<ConversionFormat>,
    mode: AnalysisMode,
) -> Result<AnalyzeReport, CliError> {
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

    let parsed =
        parse(&ParseRequest::new(bytes.into(), format), &ConverterAdapter).map_err(|error| {
            CliError::Parse {
                source_name: input.name().to_owned(),
                error,
            }
        })?;
    let response = analyze(
        &AnalysisRequest::new(parsed.circuit(), mode),
        &AnalyzerAdapter,
    );

    Ok(AnalyzeReport::new(
        input.name().to_owned(),
        None,
        format_name(format),
        response.result(),
    ))
}

fn analyze_python(
    input: &Input,
    selector: Option<&str>,
    mode: AnalysisMode,
    reporter: &mut Reporter,
    circuits: &mut usize,
    errors: &mut usize,
) {
    let bytes = match input.read() {
        Ok(bytes) => bytes,
        Err(error) => {
            *errors += 1;
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
            *errors += 1;
            reporter.error(&error);
            return;
        }
    };

    for outcome in outcomes {
        match outcome {
            Ok(info) => {
                let response = analyze(
                    &AnalysisRequest::new(Arc::new(info.circuit().clone()), mode),
                    &AnalyzerAdapter,
                );
                let report = AnalyzeReport::new(
                    input.name().to_owned(),
                    Some(info.circuit_name().clone()),
                    "python",
                    response.result(),
                );

                *circuits += 1;
                reporter.report(&report);
            }
            Err(error) => {
                *errors += 1;
                reporter.error(&error);
            }
        }
    }
}

/// The metrics of a single analyzed circuit.
struct AnalyzeReport {
    filename: String,
    circuit_name: Option<String>,
    format: &'static str,
    metrics: AnalysisResult,
}

impl AnalyzeReport {
    const fn new(
        filename: String,
        circuit_name: Option<String>,
        format: &'static str,
        metrics: AnalysisResult,
    ) -> Self {
        Self {
            filename,
            circuit_name,
            format,
            metrics,
        }
    }

    /// The label shown in human output: `file` or `file:circuit`.
    fn label(&self) -> String {
        self.circuit_name.as_ref().map_or_else(
            || self.filename.clone(),
            |circuit_name| format!("{}:{circuit_name}", self.filename),
        )
    }
}

struct Reporter {
    output_format: OutputFormat,
    stdout: Box<dyn Write>,
    stderr: AutoStream<io::Stderr>,
    json_circuits: Vec<JsonCircuit>,
    json_errors: Vec<JsonError>,
}

impl Reporter {
    fn new(
        output_format: OutputFormat,
        color: ColorChoice,
        output: Option<&Path>,
    ) -> Result<Self, CliError> {
        let stdout: Box<dyn Write> = match output {
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
            stdout,
            stderr: AutoStream::new(io::stderr(), color),
            json_circuits: Vec::new(),
            json_errors: Vec::new(),
        })
    }

    fn report(&mut self, report: &AnalyzeReport) {
        match self.output_format {
            OutputFormat::Human => {
                let _header = writeln!(
                    self.stdout,
                    "Analyzing {} ({})",
                    report.label(),
                    report.format
                );
                let _metrics = write_metrics(&mut self.stdout, report.metrics);
                let _blank = writeln!(self.stdout);
            }
            OutputFormat::Json => self.json_circuits.push(JsonCircuit::new(report)),
        }
    }

    fn error(&mut self, error: &CliError) {
        if self.output_format == OutputFormat::Json {
            self.json_errors.push(JsonError::new(error));
        }

        output::human_error(&mut self.stderr, error);
    }

    fn finish(mut self, circuits: usize, errors: usize) {
        match self.output_format {
            OutputFormat::Human => {
                let noun = if circuits == 1 { "circuit" } else { "circuits" };
                let failed = if errors > 0 {
                    format!(", {errors} failed")
                } else {
                    String::new()
                };
                let _summary = writeln!(self.stdout, "Analyzed {circuits} {noun}{failed}");
            }
            OutputFormat::Json => {
                let report = JsonAnalyzeReport {
                    circuits: self.json_circuits,
                    errors: self.json_errors,
                };

                if let Ok(json) = serde_json::to_string_pretty(&report) {
                    let _json = writeln!(self.stdout, "{json}");
                }
            }
        }
    }
}

fn write_metrics(writer: &mut dyn Write, metrics: AnalysisResult) -> io::Result<()> {
    match metrics {
        AnalysisResult::Simple(metrics) => write_simple(writer, &metrics),
        AnalysisResult::Detailed(metrics) => write_detailed(writer, &metrics),
    }
}

fn write_simple(writer: &mut dyn Write, metrics: &AnalysisMetrics) -> io::Result<()> {
    writeln!(writer, "    qubits: {}", metrics.qubit_count)?;
    writeln!(writer, "    classical bits: {}", metrics.bit_count)?;
    writeln!(writer, "    time steps: {}", metrics.time_step_count)?;
    writeln!(writer, "    x gates: {}", metrics.x_count)?;
    writeln!(writer, "    y gates: {}", metrics.y_count)?;
    writeln!(writer, "    z gates: {}", metrics.z_count)?;
    writeln!(writer, "    pauli gates: {}", metrics.pauli_count)?;
    writeln!(writer, "    hadamard gates: {}", metrics.hadamard_count)?;
    writeln!(writer, "    rotation gates: {}", metrics.rotation_count)?;
    writeln!(writer, "    phase gates: {}", metrics.phase_count)?;
    writeln!(
        writer,
        "    square root gates: {}",
        metrics.square_root_count
    )?;
    writeln!(writer, "    t gates: {}", metrics.t_count)?;
    writeln!(writer, "    swap gates: {}", metrics.swap_count)?;
    writeln!(writer, "    cx gates: {}", metrics.cx_count)?;
    writeln!(writer, "    ccx gates: {}", metrics.ccx_count)?;
    writeln!(writer, "    operations: {}", metrics.operation_count)?;
    writeln!(writer, "    gates: {}", metrics.gate_count)?;
    writeln!(writer, "    measurements: {}", metrics.measurement_count)?;
    writeln!(
        writer,
        "    single-qubit gates: {}",
        metrics.single_gate_count
    )?;
    writeln!(
        writer,
        "    controlled gates: {}",
        metrics.controlled_gate_count
    )?;
    writeln!(
        writer,
        "    measured qubits: {}",
        metrics.measured_qubit_count
    )?;
    writeln!(
        writer,
        "    unmeasured qubits: {}",
        metrics.unmeasured_qubit_count
    )?;
    writeln!(writer, "    gate types: {}", metrics.gate_types_count)
}

fn write_detailed(writer: &mut dyn Write, metrics: &DetailedAnalysisMetrics) -> io::Result<()> {
    writeln!(writer, "    qubits: {}", metrics.qubit_count)?;
    writeln!(writer, "    time steps: {}", metrics.time_step_count)?;
    writeln!(writer, "    max density: {}", metrics.max_density)?;
    writeln!(
        writer,
        "    average density: {:.2}",
        metrics.average_density
    )?;
    writeln!(writer, "    x gates: {}", metrics.x_count)?;
    writeln!(writer, "    y gates: {}", metrics.y_count)?;
    writeln!(writer, "    z gates: {}", metrics.z_count)?;
    writeln!(writer, "    pauli gates: {}", metrics.pauli_count)?;
    writeln!(writer, "    hadamard gates: {}", metrics.hadamard_count)?;
    writeln!(
        writer,
        "    initial superposition ratio: {:.2}",
        metrics.initial_superposition_ratio
    )?;
    writeln!(
        writer,
        "    other single-qubit gates: {}",
        metrics.other_single_gate_count
    )?;
    writeln!(
        writer,
        "    single-qubit gates: {}",
        metrics.single_gate_count
    )?;
    writeln!(
        writer,
        "    single-controlled gates: {}",
        metrics.single_controlled_gate_count
    )?;
    writeln!(writer, "    swap gates: {}", metrics.swap_count)?;
    writeln!(writer, "    cx gates: {}", metrics.cx_count)?;
    writeln!(writer, "    cx qubit ratio: {:.2}", metrics.cx_qubit_ratio)?;
    writeln!(writer, "    average cx: {:.2}", metrics.average_cx)?;
    writeln!(writer, "    max cx: {}", metrics.max_cx_count)?;
    writeln!(writer, "    ccx gates: {}", metrics.ccx_count)?;
    writeln!(
        writer,
        "    ccx qubit ratio: {:.2}",
        metrics.ccx_qubit_ratio
    )?;
    writeln!(writer, "    average ccx: {:.2}", metrics.average_ccx)?;
    writeln!(writer, "    max ccx: {}", metrics.max_ccx_count)?;
    writeln!(writer, "    operations: {}", metrics.operation_count)?;
    writeln!(writer, "    gates: {}", metrics.gate_count)?;
    writeln!(writer, "    measurements: {}", metrics.measurement_count)?;
    writeln!(
        writer,
        "    controlled gates: {}",
        metrics.controlled_gate_count
    )?;
    writeln!(
        writer,
        "    single gate ratio: {:.2}",
        metrics.single_gate_ratio
    )?;
    writeln!(
        writer,
        "    measured qubits: {}",
        metrics.measured_qubit_count
    )?;
    writeln!(writer, "    measured ratio: {:.2}", metrics.measure_ratio)?;
    writeln!(
        writer,
        "    unmeasured ratio: {:.2}",
        metrics.unmeasured_qubit_ratio
    )
}

/// A single analyzed circuit of the JSON report.
#[derive(Serialize)]
struct JsonCircuit {
    filename: String,
    circuit: Option<String>,
    metrics: serde_json::Value,
}

impl JsonCircuit {
    fn new(report: &AnalyzeReport) -> Self {
        let metrics = match report.metrics {
            AnalysisResult::Simple(metrics) => serde_json::to_value(metrics),
            AnalysisResult::Detailed(metrics) => serde_json::to_value(metrics),
        }
        .expect("metrics should serialize");

        Self {
            filename: report.filename.clone(),
            circuit: report.circuit_name.clone(),
            metrics,
        }
    }
}

/// The JSON report for an `analyze` run.
#[derive(Serialize)]
struct JsonAnalyzeReport {
    circuits: Vec<JsonCircuit>,
    errors: Vec<JsonError>,
}
