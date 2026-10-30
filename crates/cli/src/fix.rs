use std::sync::Arc;

use serde::Serialize;

use qctidy_codegen::CodegenAdapter;
use qctidy_facade::{ApplyFixRequest, Diagnostic, FixContext, FixEdit, apply_fix};
use qctidy_ports::SourceLocation;

use crate::text;

/// A source edit that applies a single fix.
///
/// Internally the columns are zero-based byte offsets, matching [`SourceLocation`].
/// The JSON report converts them to UTF-16 code units, which is what editors use.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct Edit {
    line: usize,
    column: usize,
    end_line: usize,
    end_column: usize,
    new_text: String,
}

impl Edit {
    /// The same edit with columns in UTF-16 code units, as VS Code expects.
    #[must_use]
    pub(crate) fn with_utf16_columns(&self, lines: &[&str]) -> Self {
        Self {
            line: self.line,
            column: text::utf16_column(lines, self.line, self.column),
            end_line: self.end_line,
            end_column: text::utf16_column(lines, self.end_line, self.end_column),
            new_text: self.new_text.clone(),
        }
    }
}

impl From<&FixEdit> for Edit {
    fn from(edit: &FixEdit) -> Self {
        Self {
            line: edit.line(),
            column: edit.column(),
            end_line: edit.end_line(),
            end_column: edit.end_column(),
            new_text: edit.new_text().clone(),
        }
    }
}

/// Compute the edits that apply the fix described by `diagnostic`.
///
/// Returns `None` when the diagnostic has no source-level fix (for example,
/// canonicalization rules).
pub(crate) fn edits_for(
    source: Arc<str>,
    diagnostic: &Diagnostic,
    source_map: Arc<[SourceLocation]>,
    circuit_name: Arc<str>,
    context: &FixContext,
) -> Option<Vec<Edit>> {
    let request = ApplyFixRequest::new(source, diagnostic.clone(), source_map, circuit_name);
    let response = apply_fix(&request, context, &CodegenAdapter).ok()?;

    if response.edits().is_empty() {
        return None;
    }

    Some(response.edits().iter().map(Edit::from).collect())
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
