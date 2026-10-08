use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::mem;
use std::path::Path;
use std::process::ExitCode;

use anstream::{AutoStream, ColorChoice};
use qctidy::{Graph, RuleConfiguration, RuleSeverity, fixer};
use qctidy_converter::ConverterAdapter;
use qctidy_facade::{ParseRequest, Session, parse};
use qctidy_ports::ConversionFormat;

use crate::arguments::{CheckArguments, OutputFormat};
use crate::error::CliError;
use crate::input::{Input, format_name};
use crate::output::{self, CircuitReport, JsonDiagnostic, JsonError, Stats};
use crate::progress::Progress;
use crate::python;

/// Run the `check` command.
pub(crate) fn run(arguments: &CheckArguments, color: ColorChoice) -> ExitCode {
    let configuration = match build_configuration(&arguments.select, &arguments.ignore) {
        Ok(configuration) => configuration,
        Err(error) => {
            let mut stderr = AutoStream::new(io::stderr(), color);
            output::human_error(&mut stderr, &error);
            return ExitCode::from(2);
        }
    };
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
    let session = Session::new(configuration);

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
                stats.diagnostics += report.diagnostics.len();
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
            .map(|diagnostic| output::diagnostic_snippet(&graph, diagnostic.positions()))
            .collect()
    } else {
        Vec::new()
    };

    Ok(CircuitReport {
        filename: input.name().to_owned(),
        circuit_name: None,
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
                        .map(|diagnostic| {
                            output::diagnostic_snippet(&graph, diagnostic.positions())
                        })
                        .collect()
                } else {
                    Vec::new()
                };

                let report = CircuitReport {
                    filename: input.name().to_owned(),
                    circuit_name: Some(info.circuit_name),
                    format: "python",
                    qubit_count: check.qubit_count(),
                    time_step_count: check.time_step_count(),
                    gate_count: check.gate_count(),
                    diagnostics: check.diagnostics().clone(),
                    snippets: circuit_snippets,
                    source_map: Some(info.source_map),
                };

                stats.circuits += 1;
                stats.diagnostics += report.diagnostics.len();
                reporter.report(&report);
            }
            Err(error) => {
                stats.errors += 1;
                reporter.error(&error);
            }
        }
    }
}

/// Build the rule configuration from the `--select` and `--ignore` codes.
///
/// When `--select` is given, only the selected rules run; otherwise every rule
/// runs. `--ignore` always disables the given rules.
fn build_configuration(
    select: &[String],
    ignore: &[String],
) -> Result<RuleConfiguration, CliError> {
    validate_selectors(select, ignore)?;

    let mut configuration = if select.is_empty() {
        RuleConfiguration::new(RuleSeverity::Warn)
    } else {
        RuleConfiguration::new(RuleSeverity::Off)
    };

    for selector in select {
        configuration.select(selector);
    }

    for selector in ignore {
        configuration.ignore(selector);
    }

    Ok(configuration)
}

/// Check that every selector matches a known rule or category code.
fn validate_selectors(select: &[String], ignore: &[String]) -> Result<(), CliError> {
    let known: HashSet<&str> = fixer::default_rules()
        .iter()
        .flat_map(|metadata| [*metadata.code(), metadata.category().code()])
        .collect();

    for selector in select.iter().chain(ignore) {
        if !known.contains(selector.as_str()) {
            return Err(CliError::UnknownRuleSelector {
                selector: selector.clone(),
            });
        }
    }

    Ok(())
}

fn exit_code(stats: &Stats, no_fail: bool) -> ExitCode {
    if stats.errors > 0 {
        ExitCode::from(2)
    } else if stats.diagnostics > 0 && !no_fail {
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
    errors: Vec<JsonError>,
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
            errors: Vec::new(),
        })
    }

    fn report(&mut self, report: &CircuitReport) {
        match self.output_format {
            OutputFormat::Human => output::human_report(&mut self.stdout, report, self.quiet),
            OutputFormat::Json => {
                self.diagnostics
                    .extend(report.diagnostics.iter().map(|diagnostic| {
                        JsonDiagnostic::new(
                            &report.filename,
                            report.circuit_name.as_deref(),
                            diagnostic,
                            report.source_map.as_deref(),
                        )
                    }));
            }
        }
    }

    fn error(&mut self, error: &CliError) {
        if self.output_format == OutputFormat::Json {
            self.errors.push(JsonError::new(error));
        }

        output::human_error(&mut self.stderr, error);
    }

    fn finish(&mut self, stats: &Stats) {
        match self.output_format {
            OutputFormat::Human => {
                if !self.quiet {
                    output::human_summary(&mut self.stdout, stats);
                }
            }
            OutputFormat::Json => output::json_report(
                &mut self.stdout,
                mem::take(&mut self.diagnostics),
                mem::take(&mut self.errors),
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use qctidy::{RuleCategory, RuleMetadata};

    fn codes(values: &[&str]) -> Vec<String> {
        values.iter().map(|value| (*value).to_owned()).collect()
    }

    fn rule_in(category: RuleCategory) -> RuleMetadata {
        *fixer::default_rules()
            .iter()
            .find(|metadata| metadata.category() == category)
            .expect("the category should have at least one rule")
    }

    fn rule_outside(category: RuleCategory) -> RuleMetadata {
        *fixer::default_rules()
            .iter()
            .find(|metadata| metadata.category() != category)
            .expect("there should be rules outside the category")
    }

    #[test]
    fn no_selectors_enables_every_rule() {
        let configuration = build_configuration(&[], &[]).unwrap();
        let redundancy = rule_in(RuleCategory::Redundancy);

        assert_eq!(configuration.severity(&redundancy), RuleSeverity::Warn);
    }

    #[test]
    fn select_restricts_the_enabled_rules() {
        let configuration = build_configuration(&codes(&["R"]), &[]).unwrap();

        assert_eq!(
            configuration.severity(&rule_in(RuleCategory::Redundancy)),
            RuleSeverity::Warn
        );
        assert_eq!(
            configuration.severity(&rule_outside(RuleCategory::Redundancy)),
            RuleSeverity::Off
        );
    }

    #[test]
    fn ignore_disables_the_given_rules() {
        let configuration = build_configuration(&[], &codes(&["R"])).unwrap();

        assert_eq!(
            configuration.severity(&rule_in(RuleCategory::Redundancy)),
            RuleSeverity::Off
        );
        assert_eq!(
            configuration.severity(&rule_outside(RuleCategory::Redundancy)),
            RuleSeverity::Warn
        );
    }

    #[test]
    fn ignore_wins_over_select_for_the_same_selector() {
        let configuration = build_configuration(&codes(&["R"]), &codes(&["R"])).unwrap();

        assert_eq!(
            configuration.severity(&rule_in(RuleCategory::Redundancy)),
            RuleSeverity::Off
        );
    }

    #[test]
    fn a_rule_selector_wins_over_a_category_selector() {
        let redundancy = rule_in(RuleCategory::Redundancy);
        let configuration =
            build_configuration(&codes(&["R"]), &codes(&[*redundancy.code()])).unwrap();

        assert_eq!(configuration.severity(&redundancy), RuleSeverity::Off);
    }

    #[test]
    fn repeated_selectors_are_ignored() {
        let configuration = build_configuration(&codes(&["R", "R", "R"]), &[]).unwrap();

        assert_eq!(
            configuration.severity(&rule_in(RuleCategory::Redundancy)),
            RuleSeverity::Warn
        );
    }

    #[test]
    fn unknown_selectors_are_rejected() {
        let error = build_configuration(&codes(&["NOPE"]), &[]).unwrap_err();

        assert!(matches!(
            error,
            CliError::UnknownRuleSelector { selector } if selector == "NOPE"
        ));
    }
}
