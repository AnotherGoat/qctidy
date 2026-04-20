use std::f64::consts::PI;

use num_rational::Ratio;

/// A full cycle of rotation across a qubit, in radians.
///
/// Note that its value is not 2pi because that's how the rotation matrices are defined.
/// The angle is divided by 2 inside the formula.
pub(crate) const FULL_CYCLE: f64 = 4.0 * PI;

const MAX_DENOMINATOR: i64 = 16;
const RELATIVE_TOLERANCE: f64 = 1e-5;
const ABSOLUTE_TOLERANCE: f64 = 1e-8;

/// Check whether two floats are approximately equal.
///
/// Uses `NumPy`'s default tolerance values, which are 1e-5 for relative and 1e-8 for absolute.
pub(crate) fn are_floats_similar(first: f64, second: f64) -> bool {
    (first - second).abs()
        <= RELATIVE_TOLERANCE.mul_add(first.abs().max(second.abs()), ABSOLUTE_TOLERANCE)
}

pub(crate) fn are_option_floats_similar(first: Option<f64>, second: Option<f64>) -> bool {
    match (first, second) {
        (Some(first), Some(second)) => are_floats_similar(first, second),
        (None, None) => true,
        _ => false,
    }
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
pub(crate) fn rationalize_in_terms_of_pi(number: f64) -> Option<Ratio<i64>> {
    let pi_factor = number / PI;

    let mut best = None;
    let mut best_error = f64::INFINITY;

    for denominator in 1..=MAX_DENOMINATOR {
        let numerator = (pi_factor * denominator as f64).round() as i64;
        let ratio = Ratio::new(numerator, denominator);

        let approximation = *ratio.numer() as f64 / *ratio.denom() as f64;
        let error = (approximation - pi_factor).abs();

        if error < best_error {
            best_error = error;
            best = Some(ratio);
        }
    }

    let result = best?;
    let final_approximation = *result.numer() as f64 / *result.denom() as f64;

    are_floats_similar(final_approximation, pi_factor).then_some(result)
}
