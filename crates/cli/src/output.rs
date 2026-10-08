use std::fmt::{self, Write as _};
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use anstyle::{AnsiColor, Color, Style};
use qctidy::{Graph, Position, RuleSeverity};
use qctidy_facade::Diagnostic;
use qctidy_ports::SourceLocation;
use serde::Serialize;

use crate::error::CliError;

const MAX_POSITIONS: usize = 8;

const SNIPPET_MARGIN: usize = 1;
const SNIPPET_MAX_ROWS: usize = 10;
const SNIPPET_MAX_COLUMNS: usize = 20;
const SNIPPET_MAX_POSITIONS: usize = 64;

const HEADER: Style = Style::new()
    .bold()
    .fg_color(Some(Color::Ansi(AnsiColor::Green)));
const SUCCESS: Style = Style::new().fg_color(Some(Color::Ansi(AnsiColor::Green)));
const ERROR: Style = Style::new()
    .bold()
    .fg_color(Some(Color::Ansi(AnsiColor::Red)));
const WARN: Style = Style::new().fg_color(Some(Color::Ansi(AnsiColor::Yellow)));
const RULE: Style = Style::new().bold();
const CATEGORY: Style = Style::new().dimmed();
const DIM: Style = Style::new().dimmed();
const POSITION: Style = Style::new().fg_color(Some(Color::Ansi(AnsiColor::Cyan)));
const SUMMARY: Style = Style::new().bold();
const MATCH: Style = Style::new()
    .bold()
    .fg_color(Some(Color::Ansi(AnsiColor::Yellow)));

/// Aggregated results for every checked circuit.
#[derive(Debug, Default)]
#[expect(clippy::field_scoped_visibility_modifiers)]
pub(crate) struct Stats {
    pub(crate) circuits: usize,
    pub(crate) diagnostics: usize,
    pub(crate) errors: usize,
}

/// The check results for a single circuit.
#[derive(Debug)]
#[expect(clippy::field_scoped_visibility_modifiers)]
pub(crate) struct CircuitReport {
    /// The source file the circuit came from.
    pub(crate) filename: String,
    /// The circuit's variable name, only filled for Python inputs.
    pub(crate) circuit_name: Option<String>,
    pub(crate) format: &'static str,
    pub(crate) qubit_count: usize,
    pub(crate) time_step_count: usize,
    pub(crate) gate_count: usize,
    pub(crate) diagnostics: Vec<Diagnostic>,
    /// One optional snippet per diagnostic, only filled when snippets are enabled.
    pub(crate) snippets: Vec<Option<String>>,
    /// Maps operation indices to source ranges, only filled for Python inputs.
    pub(crate) source_map: Option<Vec<SourceLocation>>,
}

impl CircuitReport {
    /// The label shown in human output: `file` or `file:circuit`.
    fn label(&self) -> String {
        self.circuit_name.as_ref().map_or_else(
            || self.filename.clone(),
            |circuit_name| format!("{}:{circuit_name}", self.filename),
        )
    }
}

/// Write a human-readable report for a single circuit.
pub(crate) fn human_report(writer: &mut dyn Write, report: &CircuitReport, quiet: bool) {
    if quiet {
        for diagnostic in &report.diagnostics {
            let metadata = diagnostic.metadata();

            let _quiet_result = writeln!(
                writer,
                "{}: {}{} {}: {} {}",
                paint(DIM, &report.label()),
                paint_severity(diagnostic.severity()),
                paint(RULE, &format!("[{}]", metadata.code())),
                paint(CATEGORY, &format!("({})", metadata.category())),
                metadata.description(),
                paint(DIM, &format!("at {}", format_location(report, diagnostic))),
            );
        }

        return;
    }

    let _header_result = writeln!(
        writer,
        "{} {} {}",
        paint(HEADER, "Checking"),
        report.label(),
        paint(DIM, &format!("({})", report.format)),
    );
    let _stats_result = writeln!(
        writer,
        "  {}",
        paint(
            DIM,
            &format!(
                "{} {}, {} {}, {} {}",
                report.qubit_count,
                plural(report.qubit_count, "qubit", "qubits"),
                report.time_step_count,
                plural(report.time_step_count, "time step", "time steps"),
                report.gate_count,
                plural(report.gate_count, "gate", "gates"),
            ),
        ),
    );

    for (index, diagnostic) in report.diagnostics.iter().enumerate() {
        let metadata = diagnostic.metadata();

        let _rule_result = writeln!(
            writer,
            "  {}{} {}: {}",
            paint_severity(diagnostic.severity()),
            paint(RULE, &format!("[{}]", metadata.code())),
            paint(CATEGORY, &format!("({})", metadata.category())),
            metadata.description(),
        );
        let _positions_result = writeln!(
            writer,
            "    {} {}",
            paint(DIM, "at"),
            paint(POSITION, &format_location(report, diagnostic)),
        );

        if let Some(snippet) = report.snippets.get(index).and_then(Option::as_ref) {
            let _snippet_result = writeln!(writer, "{snippet}");
        }
    }

    let _blank_result = writeln!(writer);
}

/// Write a human-readable error to a stream.
pub(crate) fn human_error(writer: &mut dyn Write, error: &dyn fmt::Display) {
    let _error_result = writeln!(writer, "{} {error}", paint(ERROR, "error:"));
}

/// Write the summary of a human-readable run.
pub(crate) fn human_summary(writer: &mut dyn Write, stats: &Stats) {
    let mut text = String::new();

    if stats.circuits == 0 {
        text.push_str("No circuits checked");
    } else {
        let _circuits_result = write!(
            text,
            "Checked {} {}",
            stats.circuits,
            plural(stats.circuits, "circuit", "circuits")
        );
    }

    if stats.diagnostics > 0 {
        let _diagnostics_result = write!(
            text,
            ", {} {} found",
            stats.diagnostics,
            plural(stats.diagnostics, "diagnostic", "diagnostics")
        );
    } else if stats.errors == 0 {
        text.push_str(": no fixable patterns found");
    } else {
        // Failures are already reported on standard error.
    }

    if stats.errors > 0 {
        let _failed_result = write!(text, ", {} failed", stats.errors);
    }

    let style = if stats.diagnostics > 0 {
        SUMMARY
    } else {
        SUCCESS
    };

    let _summary_result = writeln!(writer, "{}", paint(style, &text));
}

/// Write bytes to a file, or to standard output when no target is given.
pub(crate) fn write_bytes(target: Option<&Path>, bytes: &[u8]) -> Result<(), CliError> {
    target.map_or_else(
        || write_to_stdout(bytes),
        |path| {
            fs::write(path, bytes).map_err(|error| CliError::Write {
                target_name: path.display().to_string(),
                error,
            })
        },
    )
}

/// The file path for the `index`-th output, inserting `_index` before the file extension.
#[must_use]
pub(crate) fn numbered_path(path: &Path, index: usize) -> PathBuf {
    let file_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("output");

    let numbered = match file_name.rsplit_once('.') {
        Some((stem, extension)) if !stem.is_empty() => format!("{stem}_{index}.{extension}"),
        _ => format!("{file_name}_{index}"),
    };

    path.with_file_name(numbered)
}

/// Write one or more outputs: a single output to the target (or standard
/// output), or numbered files when there are several.
pub(crate) fn write_outputs(target: Option<&Path>, outputs: &[Vec<u8>]) -> Result<(), CliError> {
    match outputs.len() {
        0 => Ok(()),
        1 => write_bytes(target, &outputs[0]),
        _ => {
            let target = target.ok_or(CliError::MultipleOutputs)?;

            for (index, bytes) in outputs.iter().enumerate() {
                let path = numbered_path(target, index);
                write_bytes(Some(&path), bytes)?;
            }

            Ok(())
        }
    }
}

fn write_to_stdout(bytes: &[u8]) -> Result<(), CliError> {
    let mut stdout = io::stdout().lock();

    stdout
        .write_all(bytes)
        .and_then(|()| stdout.flush())
        .map_err(|error| CliError::Write {
            target_name: "<stdout>".to_owned(),
            error,
        })
}

/// Render a small circuit snippet around the given positions, when there are few enough.
#[must_use]
pub(crate) fn diagnostic_snippet(graph: &Graph, positions: &[Position]) -> Option<String> {
    if positions.is_empty() || positions.len() > SNIPPET_MAX_POSITIONS {
        return None;
    }

    let min_row = positions.iter().map(Position::row).min()?;
    let max_row = positions.iter().map(Position::row).max()?;
    let min_column = positions.iter().map(Position::column).min()?;
    let max_column = positions.iter().map(Position::column).max()?;

    let first_row = min_row.saturating_sub(SNIPPET_MARGIN);
    let last_row = (max_row + SNIPPET_MARGIN).min(graph.height().saturating_sub(1));
    let first_column = min_column.saturating_sub(SNIPPET_MARGIN);
    let last_column = (max_column + SNIPPET_MARGIN).min(graph.width().saturating_sub(1));

    if last_row - first_row + 1 > SNIPPET_MAX_ROWS
        || last_column - first_column + 1 > SNIPPET_MAX_COLUMNS
    {
        return None;
    }

    let mut cells: Vec<Vec<String>> = Vec::new();
    let mut cell_width = 2;

    for row in first_row..=last_row {
        let mut line = Vec::new();

        for column in first_column..=last_column {
            let position = Position::new(row, column);
            let text = graph.get_node(position).map_or_else(String::new, |node| {
                let name = node.r#type().to_string().to_ascii_uppercase();

                if positions.contains(&position) {
                    format!("[{name}]")
                } else {
                    name
                }
            });

            cell_width = cell_width.max(text.len());
            line.push(text);
        }

        cells.push(line);
    }

    let label_width = (first_row..=last_row)
        .map(|row| row.to_string().len() + 1)
        .max()
        .unwrap_or(2);

    let mut output = String::new();
    let _header_result = write!(output, "{}", " ".repeat(4 + label_width + 1));

    for column in first_column..=last_column {
        let _column_result = write!(output, "{column:>cell_width$} ");
    }

    output.push('\n');

    for (row, line) in (first_row..=last_row).zip(cells.iter()) {
        let label = format!("q{row}");
        let _row_result = write!(output, "    {label:<label_width$} ");

        for (offset, text) in line.iter().enumerate() {
            let column = first_column + offset;

            if offset > 0 {
                output.push(' ');
            }

            let padded = format!("{text:<cell_width$}");

            if positions.contains(&Position::new(row, column)) {
                let _match_result = write!(output, "{}", paint(MATCH, &padded));
            } else {
                output.push_str(&padded);
            }
        }

        output.push('\n');
    }

    output.pop();
    Some(output)
}

/// A single diagnostic of the JSON report.
#[derive(Debug, Serialize)]
pub(crate) struct JsonDiagnostic {
    filename: String,
    circuit: Option<String>,
    code: &'static str,
    category: String,
    severity: String,
    message: &'static str,
    circuit_positions: Vec<JsonCircuitPosition>,
    operation_indices: Vec<usize>,
    source_locations: Vec<JsonSourceLocation>,
}

/// A circuit that could not be checked, reported in the JSON report.
#[derive(Debug, Serialize)]
pub(crate) struct JsonError {
    filename: String,
    circuit: Option<String>,
    message: String,
}

impl JsonError {
    /// Build the JSON error for a CLI error, keeping the circuit when known.
    #[expect(clippy::wildcard_enum_match_arm, clippy::pattern_type_mismatch)]
    pub(crate) fn new(error: &CliError) -> Self {
        match error {
            CliError::UncheckableCircuit {
                filename,
                circuit,
                message,
            } => Self {
                filename: filename.clone(),
                circuit: Some(circuit.clone()),
                message: message.clone(),
            },
            _ => Self {
                filename: String::new(),
                circuit: None,
                message: error.to_string(),
            },
        }
    }
}

/// The JSON report for a `check` run.
#[derive(Debug, Serialize)]
pub(crate) struct JsonReport {
    diagnostics: Vec<JsonDiagnostic>,
    errors: Vec<JsonError>,
}

#[derive(Debug, Serialize)]
struct JsonCircuitPosition {
    row: usize,
    column: usize,
}

#[derive(Debug, Serialize)]
struct JsonSourceLocation {
    line: usize,
    column: usize,
    end_line: usize,
    end_column: usize,
}

impl JsonDiagnostic {
    pub(crate) fn new(
        filename: &str,
        circuit: Option<&str>,
        diagnostic: &Diagnostic,
        source_map: Option<&[SourceLocation]>,
    ) -> Self {
        let metadata = diagnostic.metadata();

        Self {
            filename: filename.to_owned(),
            circuit: circuit.map(str::to_owned),
            code: metadata.code(),
            category: metadata.category().to_string(),
            severity: diagnostic.severity().to_string(),
            message: metadata.description(),
            circuit_positions: diagnostic
                .positions()
                .iter()
                .map(|position| JsonCircuitPosition {
                    row: position.row(),
                    column: position.column(),
                })
                .collect(),
            operation_indices: diagnostic.operation_indices().clone(),
            source_locations: source_map.map_or_else(Vec::new, |source_map| {
                diagnostic
                    .operation_indices()
                    .iter()
                    .filter_map(|index| source_map.get(*index))
                    .map(|location| JsonSourceLocation {
                        line: location.line,
                        column: location.column,
                        end_line: location.end_line,
                        end_column: location.end_column,
                    })
                    .collect()
            }),
        }
    }
}

/// Write the JSON report to a stream.
pub(crate) fn json_report(
    writer: &mut dyn Write,
    diagnostics: Vec<JsonDiagnostic>,
    errors: Vec<JsonError>,
) {
    let report = JsonReport {
        diagnostics,
        errors,
    };

    if let Ok(json) = serde_json::to_string_pretty(&report) {
        let _json_result = writeln!(writer, "{json}");
    }
}

/// Render text with an ANSI style.
pub(crate) fn paint(style: Style, text: &str) -> String {
    format!("{}{text}{}", style.render(), style.render_reset())
}

/// Render the severity of a diagnostic as styled text.
fn paint_severity(severity: RuleSeverity) -> String {
    let style = match severity {
        RuleSeverity::Error => ERROR,
        RuleSeverity::Warn | RuleSeverity::Off => WARN,
    };

    paint(style, &severity.to_string())
}

fn format_positions(positions: &[Position]) -> String {
    let mut text = String::new();

    for position in positions.iter().take(MAX_POSITIONS) {
        if !text.is_empty() {
            text.push_str(", ");
        }

        let _position_result = write!(text, "({}, {})", position.row(), position.column());
    }

    if positions.len() > MAX_POSITIONS {
        let _more_result = write!(text, ", and {} more", positions.len() - MAX_POSITIONS);
    }

    text
}

/// Format a diagnostic location: source ranges for Python, circuit coordinates otherwise.
fn format_location(report: &CircuitReport, diagnostic: &Diagnostic) -> String {
    report.source_map.as_ref().map_or_else(
        || format_positions(diagnostic.positions()),
        |source_map| format_source_locations(diagnostic.operation_indices(), source_map),
    )
}

fn format_source_locations(operation_indices: &[usize], source_map: &[SourceLocation]) -> String {
    let mut text = String::new();

    for index in operation_indices.iter().take(MAX_POSITIONS) {
        if let Some(location) = source_map.get(*index) {
            if !text.is_empty() {
                text.push_str(", ");
            }

            let _position_result = write!(text, "{}:{}", location.line, location.column);
        }
    }

    if operation_indices.len() > MAX_POSITIONS {
        let _more_result = write!(
            text,
            ", and {} more",
            operation_indices.len() - MAX_POSITIONS
        );
    }

    text
}

const fn plural(count: usize, singular: &'static str, many: &'static str) -> &'static str {
    if count == 1 { singular } else { many }
}
