use std::collections::BTreeMap;
use std::io::{self, Write};
use std::process::ExitCode;

use anstream::{AutoStream, ColorChoice};
use anstyle::{AnsiColor, Color, Style};
use qctidy::{RuleCategory, RuleMetadata, fixer};

use crate::output;

const CATEGORY: Style = Style::new()
    .bold()
    .fg_color(Some(Color::Ansi(AnsiColor::Green)));
const CODE: Style = Style::new().fg_color(Some(Color::Ansi(AnsiColor::Cyan)));
const DESCRIPTION: Style = Style::new().dimmed();

/// Run the `rules` command.
pub(crate) fn run(color: ColorChoice) -> ExitCode {
    let mut rules_by_category: BTreeMap<&'static str, (RuleCategory, Vec<RuleMetadata>)> =
        BTreeMap::new();

    for metadata in fixer::default_rules() {
        let category = metadata.category();

        rules_by_category
            .entry(category.code())
            .or_insert_with(|| (category, Vec::new()))
            .1
            .push(metadata);
    }

    let code_width = rules_by_category
        .values()
        .flat_map(|entry| &entry.1)
        .map(|metadata| metadata.code().len())
        .max()
        .unwrap_or(0);

    let mut stdout = AutoStream::new(io::stdout(), color);

    for (index, (category_code, (category, rules))) in rules_by_category.into_iter().enumerate() {
        if index > 0 {
            let _blank_result = writeln!(stdout);
        }

        let _category_result = writeln!(
            stdout,
            "{}",
            output::paint(CATEGORY, &format!("{category_code} ({category})"))
        );

        for metadata in rules {
            let padded_code = format!("{:<code_width$}", metadata.code());

            let _rule_result = writeln!(
                stdout,
                "  {}  {}",
                output::paint(CODE, &padded_code),
                output::paint(DESCRIPTION, metadata.description()),
            );
        }
    }

    ExitCode::SUCCESS
}
