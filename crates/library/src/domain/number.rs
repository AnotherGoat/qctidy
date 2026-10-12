/// Truncate a 64-bit signed integer to a 64-bit float.
///
/// There are no guarantees that the conversion is lossless, especially for bigger values.
#[must_use]
#[expect(clippy::cast_precision_loss)]
pub const fn truncate_i64_to_f64(value: i64) -> f64 {
    value as f64
}

/// Truncate a 64-bit unsigned integer to a 64-bit float.
///
/// There are no guarantees that the conversion is lossless, especially for bigger values.
#[must_use]
#[expect(clippy::cast_precision_loss)]
pub const fn truncate_u64_to_f64(value: u64) -> f64 {
    value as f64
}

/// Truncate a pointer-sized unsigned integer to a 64-bit float.
///
/// There are no guarantees that the conversion is lossless, especially for bigger values.
#[must_use]
#[expect(clippy::cast_precision_loss)]
pub const fn truncate_usize_to_f64(value: usize) -> f64 {
    value as f64
}

/// Truncate a 64-bit float to a 64-bit signed integer.
///
/// The value is rounded at the start, and there are no guarantees that the conversion is lossless, especially for bigger values.
#[must_use]
#[expect(clippy::cast_possible_truncation)]
pub const fn truncate_f64_to_i64(value: f64) -> i64 {
    value.round() as i64
}

/// Truncate a 64-bit float to a pointer-sized unsigned integer.
///
/// The fractional part is discarded and negative values wrap around, so the result is only meaningful for non-negative values.
#[must_use]
#[expect(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
pub const fn truncate_f64_to_usize(value: f64) -> usize {
    value as usize
}

/// Truncate a pointer-sized unsigned integer to a pointer-sized signed integer.
///
/// There are no guarantees that the conversion is lossless for values bigger than `isize::MAX`.
#[must_use]
#[expect(clippy::cast_possible_wrap)]
pub const fn truncate_usize_to_isize(value: usize) -> isize {
    value as isize
}

/// Truncate a pointer-sized unsigned integer to an 8-bit unsigned integer.
///
/// There are no guarantees that the conversion is lossless for values bigger than `u8::MAX`.
#[must_use]
#[expect(clippy::cast_possible_truncation)]
pub const fn truncate_usize_to_u8(value: usize) -> u8 {
    value as u8
}

/// Truncate a pointer-sized unsigned integer to a 16-bit unsigned integer.
///
/// There are no guarantees that the conversion is lossless for values bigger than `u16::MAX`.
#[must_use]
#[expect(clippy::cast_possible_truncation)]
pub const fn truncate_usize_to_u16(value: usize) -> u16 {
    value as u16
}
