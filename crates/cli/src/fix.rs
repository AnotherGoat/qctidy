use serde::Serialize;

use qctidy::{Circuit, Graph, fixer};
use qctidy_facade::Diagnostic;
use qctidy_ports::SourceLocation;

/// A source edit that applies a single fix.
///
/// Positions use the same convention as [`SourceLocation`]: zero-based line and
/// byte column, with an exclusive end.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct Edit {
    line: usize,
    column: usize,
    end_line: usize,
    end_column: usize,
    new_text: String,
}

/// Compute the edits that apply the fix described by `diagnostic`.
///
/// Returns `None` when the diagnostic has no source-level fix (for example,
/// canonicalization rules).
pub(crate) fn edits_for(
    source: &str,
    circuit: &Circuit,
    diagnostic: &Diagnostic,
    source_map: &[SourceLocation],
    circuit_name: &str,
) -> Option<Vec<Edit>> {
    let library_diagnostic = fixer::Diagnostic::new(
        diagnostic.metadata(),
        diagnostic.severity(),
        diagnostic.positions().clone(),
    );

    let mut graph = Graph::from(circuit);
    let replacements = fixer::apply(&mut graph, &library_diagnostic)?;

    let removed: Vec<&SourceLocation> = diagnostic
        .operation_indices()
        .iter()
        .filter_map(|index| source_map.get(*index))
        .collect();

    let (start_line, start_column) = removed
        .iter()
        .map(|location| (location.line, location.column))
        .min()?;
    let (end_line, end_column) = removed
        .iter()
        .map(|location| (location.end_line, location.end_column))
        .max()?;

    let rendered = replacements
        .iter()
        .map(|operation| qctidy_codegen::render_operation(operation, circuit_name))
        .collect::<Vec<_>>()
        .join("\n");
    let new_text = reindent(source, start_line, start_column, &rendered);

    Some(vec![Edit {
        line: start_line,
        column: start_column,
        end_line,
        end_column,
        new_text,
    }])
}

/// Re-indent a multi-line replacement to match the indentation of the replaced region.
fn reindent(source: &str, start_line: usize, start_column: usize, new_text: &str) -> String {
    if !new_text.contains('\n') {
        return new_text.to_owned();
    }

    let indent = line_indentation(source, start_line, start_column);
    new_text.replace('\n', &format!("\n{indent}"))
}

/// The leading whitespace before `column` on the given source line.
fn line_indentation(source: &str, line: usize, column: usize) -> String {
    let line_text = source.split_inclusive('\n').nth(line).unwrap_or("");
    let prefix = line_text.get(..column).unwrap_or(line_text);

    if prefix.chars().all(char::is_whitespace) {
        prefix.to_owned()
    } else {
        String::new()
    }
}

/// Apply a set of non-overlapping edits to `source`, returning the new source.
pub(crate) fn apply_edits(source: &str, edits: &[Edit]) -> String {
    let mut spans: Vec<(usize, usize, &str)> = edits
        .iter()
        .map(|edit| {
            let start = byte_offset(source, edit.line, edit.column);
            let end = byte_offset(source, edit.end_line, edit.end_column);

            (start, end, edit.new_text.as_str())
        })
        .collect();

    // Apply from the end to the beginning so earlier offsets stay valid.
    spans.sort_unstable_by_key(|(start, _, _)| *start);

    let mut result = source.to_owned();

    for (start, end, new_text) in spans.into_iter().rev() {
        result.replace_range(start..end, new_text);
    }

    result
}

/// The byte offset of a zero-based line and byte column.
fn byte_offset(source: &str, line: usize, column: usize) -> usize {
    source
        .split_inclusive('\n')
        .take(line)
        .map(str::len)
        .sum::<usize>()
        + column
}
