use std::collections::BTreeMap;
use std::io::{self, Write};
use std::process::ExitCode;

use anstream::{AutoStream, ColorChoice};
use anstyle::{AnsiColor, Color, Style};
use qctidy::{RuleMetadata, simplifier};

use crate::output;

const GROUP: Style = Style::new()
    .bold()
    .fg_color(Some(Color::Ansi(AnsiColor::Green)));
const ID: Style = Style::new().fg_color(Some(Color::Ansi(AnsiColor::Cyan)));
const DESCRIPTION: Style = Style::new().dimmed();

/// Run the `rules` command.
pub(crate) fn run(color: ColorChoice) -> ExitCode {
    let mut rules_by_group: BTreeMap<String, Vec<RuleMetadata>> = BTreeMap::new();

    for metadata in simplifier::default_rules() {
        rules_by_group
            .entry(metadata.group().to_string())
            .or_default()
            .push(metadata);
    }

    let id_width = rules_by_group
        .values()
        .flatten()
        .map(|metadata| metadata.id().len())
        .max()
        .unwrap_or(0);

    let mut stdout = AutoStream::new(io::stdout(), color);

    for (index, (group, rules)) in rules_by_group.iter().enumerate() {
        if index > 0 {
            let _blank_result = writeln!(stdout);
        }

        let _group_result = writeln!(stdout, "{}", output::paint(GROUP, group));

        for metadata in rules {
            let padded_id = format!("{:<id_width$}", metadata.id());

            let _rule_result = writeln!(
                stdout,
                "  {}  {}",
                output::paint(ID, &padded_id),
                output::paint(DESCRIPTION, metadata.description()),
            );
        }
    }

    ExitCode::SUCCESS
}
