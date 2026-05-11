use std::f64::consts::PI;

use faer::{Mat, complex::Complex64};
use num_rational::Ratio;

use crate::domain::number;

/// A full cycle of rotation across a qubit, in radians.
///
/// Note that its value is not 2pi because that's how the rotation matrices are defined.
/// The angle is divided by 2 inside the formula.
pub(crate) const FULL_CYCLE: f64 = 4.0 * PI;
/// Relative tolerance used for float comparisons, the same value as `NumPy`'s default.
pub const RELATIVE_TOLERANCE: f64 = 1e-5;
/// Absolute tolerance used for float comparisons, the same value as `NumPy`'s default.
pub const ABSOLUTE_TOLERANCE: f64 = 1e-8;
/// A very small value, used to check if a float is close to zero.
pub const EPSILON: f64 = ABSOLUTE_TOLERANCE;

const MAX_DENOMINATOR: i64 = 16;

/// Check whether two floats are approximately equal.
///
/// Uses `NumPy`'s default tolerance values, which are 1e-5 for relative and 1e-8 for absolute.
#[expect(clippy::float_cmp)]
#[must_use]
pub fn are_floats_equal(first: f64, second: f64) -> bool {
    if first == second {
        return true;
    }

    let difference = (first - second).abs();
    let scale = first.abs().max(second.abs());

    difference <= RELATIVE_TOLERANCE.mul_add(scale, ABSOLUTE_TOLERANCE)
}

/// Check whether two floats wrapped in an `Option` are approximately equal.
///
/// Uses `NumPy`'s default tolerance values, which are 1e-5 for relative and 1e-8 for absolute.
#[must_use]
pub(crate) fn are_option_floats_equal(first: Option<f64>, second: Option<f64>) -> bool {
    match (first, second) {
        (Some(first), Some(second)) => are_floats_equal(first, second),
        (None, None) => true,
        _ => false,
    }
}

/// Check whether two matrices are approximately equal.
///
/// Uses `NumPy`'s default tolerance values, which are 1e-5 for relative and 1e-8 for absolute.
#[must_use]
pub(crate) fn are_matrices_equal(first: &Mat<Complex64>, second: &Mat<Complex64>) -> bool {
    if first.nrows() != second.nrows() || first.ncols() != second.ncols() {
        return false;
    }

    for row in 0..first.nrows() {
        for column in 0..first.ncols() {
            let first_element = first[(row, column)];
            let second_element = second[(row, column)];

            if first_element == second_element {
                continue;
            }

            let difference = (first_element - second_element).norm();
            let scale = first_element.norm().max(second_element.norm());

            if difference > RELATIVE_TOLERANCE.mul_add(scale, ABSOLUTE_TOLERANCE) {
                return false;
            }
        }
    }

    true
}

/// Normalize angle to [0, `full_cycle`).
///
/// For example, 1.5 * pi becomes 3 * pi / 2.
/// Returns an error if the angle is not finite.
pub(crate) fn normalize_angle(angle: f64, full_cycle: f64) -> Result<f64, &'static str> {
    if !angle.is_finite() {
        return Err("The angle must be a finite number (not Inf or NaN)");
    }

    let mut result = angle % full_cycle;

    if result < 0.0_f64 {
        result += full_cycle;
    }

    Ok(result)
}

/// Rationalize number as a multiple of pi.
///
/// Returns None if the number is not close to being a multiple of pi.
#[must_use]
pub(crate) fn rationalize_in_terms_of_pi(number: f64) -> Option<Ratio<i64>> {
    if !number.is_finite() {
        return None;
    }

    let pi_factor = number / PI;

    let mut best = Ratio::new(number::truncate_f64_to_i64(pi_factor), 1);
    let mut best_error = f64::INFINITY;

    for denominator in 2..=MAX_DENOMINATOR {
        let numerator =
            number::truncate_f64_to_i64(pi_factor * number::truncate_i64_to_f64(denominator));
        let ratio = Ratio::new(numerator, denominator);

        let float_numerator = number::truncate_i64_to_f64(*ratio.numer());
        let float_denominator = number::truncate_i64_to_f64(*ratio.denom());
        let approximation = float_numerator / float_denominator;

        let error = (approximation - pi_factor).abs();

        if error < EPSILON {
            return Some(ratio);
        }

        if error < best_error {
            best_error = error;
            best = ratio;
        }
    }

    let float_numerator = number::truncate_i64_to_f64(*best.numer());
    let float_denominator = number::truncate_i64_to_f64(*best.denom());
    let final_approximation = float_numerator / float_denominator;

    are_floats_equal(final_approximation, pi_factor).then_some(best)
}
