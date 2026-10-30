//! Coordinate conversions for editor-facing output.

/// Split source text into lines, without the line terminators.
#[must_use]
pub(crate) fn lines(source: &str) -> Vec<&str> {
    source.split('\n').collect()
}

/// Convert a zero-based byte column into a UTF-16 code unit column.
///
/// Falls back to the byte column when the line or the boundary is missing.
#[must_use]
pub(crate) fn utf16_column(lines: &[&str], line: usize, byte_column: usize) -> usize {
    lines
        .get(line)
        .and_then(|text| text.get(..byte_column))
        .map_or(byte_column, |prefix| prefix.encode_utf16().count())
}
