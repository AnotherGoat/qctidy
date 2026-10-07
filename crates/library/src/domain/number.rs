/// Truncate a 64-bit signed integer to a 64-bit float.
///
/// There are no guarantees that the convertion is lossless, especially for bigger values.
#[must_use]
#[expect(clippy::as_conversions, clippy::cast_precision_loss)]
pub(crate) const fn truncate_i64_to_f64(value: i64) -> f64 {
    value as f64
}

/// Truncate a 64-bit float to a 64-bit signed integer.
///
/// The value is rounded at the start, and there are no guarantees that the convertion is lossless, especially for bigger values.
#[must_use]
#[expect(clippy::as_conversions, clippy::cast_possible_truncation)]
pub(crate) const fn truncate_f64_to_i64(value: f64) -> i64 {
    value.round() as i64
}
