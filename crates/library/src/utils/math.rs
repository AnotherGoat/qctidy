use std::f64::consts::PI;

use num_rational::Ratio;

pub(crate) const FULL_CYCLE: f64 = 4.0 * PI;

const MAX_DENOMINATOR: i64 = 16;
const RELATIVE_TOLERANCE: f64 = 1e-5;
const ABSOLUTE_TOLERANCE: f64 = 1e-8;

/// Check whether two floats are approximately equal.
///
/// Uses NumPy's default tolerance values, which are 1e-5 for relative and 1e-8 for absolute.
pub(crate) fn are_floats_similar(first: f64, second: f64) -> bool {
    (first - second).abs()
        <= ABSOLUTE_TOLERANCE + RELATIVE_TOLERANCE * first.abs().max(second.abs())
}

pub(crate) fn are_option_floats_similar(first: Option<f64>, second: Option<f64>) -> bool {
    match (first, second) {
        (Some(first), Some(second)) => are_floats_similar(first, second),
        (None, None) => true,
        _ => false,
    }
}

/// Normalize angle to [0, full_cycle).
///
/// For example, 1.5 * pi becomes 3 * pi / 2.
/// Returns an error if the angle is not finite.
pub(crate) fn normalize_angle(angle: f64, full_cycle: f64) -> Result<f64, &'static str> {
    if !angle.is_finite() {
        return Err("The angle must be a finite number (not Inf or NaN)");
    }

    let mut result = angle % full_cycle;

    if result < 0.0 {
        result += full_cycle;
    }

    Ok(result)
}

/// Rationalize number as a multiple of π.
///
/// Returns None if the number is not close to being a multiple of π.
pub(crate) fn rationalize_in_terms_of_pi(number: f64) -> Option<Ratio<i64>> {
    let pi_factor = number / PI;

    let mut best = None;
    let mut best_error = f64::INFINITY;

    for denominator in 1..=MAX_DENOMINATOR {
        let numerator = (pi_factor * denominator as f64).round() as i64;
        let ratio = Ratio::new(numerator, denominator);

        let approximation = *ratio.numer() as f64 / *ratio.denom() as f64;
        let error = (approximation as f64 - pi_factor).abs();

        if error < best_error {
            best_error = error;
            best = Some(ratio);
        }
    }

    let result = best?;
    let final_approximation = *result.numer() as f64 / *result.denom() as f64;

    if are_floats_similar(final_approximation, pi_factor) {
        Some(result)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn are_different_floats_similar() {
        assert!(!are_floats_similar(5.0, 10.0));
        assert!(!are_floats_similar(-3.0, 3.0));
    }

    #[test]
    fn are_close_floats_similar() {
        assert!(!are_floats_similar(0.0, 1.0));
        assert!(!are_floats_similar(0.0, 1e-1));
        assert!(!are_floats_similar(0.0, 1e-2));
        assert!(!are_floats_similar(0.0, 1e-3));
        assert!(!are_floats_similar(0.0, 1e-4));
        assert!(!are_floats_similar(0.0, 1e-5));
        assert!(!are_floats_similar(0.0, 1e-6));
        assert!(!are_floats_similar(0.0, 1e-7));
    }

    #[test]
    fn are_very_close_floats_similar() {
        assert!(are_floats_similar(0.0, 1e-8));
        assert!(are_floats_similar(0.3, 0.30000001));
        assert!(are_floats_similar(3.14159, PI));
    }

    #[test]
    fn normalize_zero_angle() {
        assert!(are_floats_similar(
            normalize_angle(0.0, 4.0 * PI).unwrap(),
            0.0
        ));
        assert!(are_floats_similar(
            normalize_angle(-0.0, 4.0 * PI).unwrap(),
            0.0
        ));
    }

    #[test]
    fn normalize_positive_angle() {
        assert!(are_floats_similar(
            normalize_angle(PI, 4.0 * PI).unwrap(),
            PI
        ));
        assert!(are_floats_similar(
            normalize_angle(2.0 * PI, 4.0 * PI).unwrap(),
            2.0 * PI
        ));
        assert!(are_floats_similar(
            normalize_angle(3.0 * PI, 4.0 * PI).unwrap(),
            3.0 * PI
        ));
        assert!(are_floats_similar(
            normalize_angle(4.0 * PI, 4.0 * PI).unwrap(),
            0.0
        ));
        assert!(are_floats_similar(
            normalize_angle(5.0 * PI, 4.0 * PI).unwrap(),
            PI
        ));
        assert!(are_floats_similar(
            normalize_angle(6.0 * PI, 4.0 * PI).unwrap(),
            2.0 * PI
        ));
        assert!(are_floats_similar(
            normalize_angle(7.0 * PI, 4.0 * PI).unwrap(),
            3.0 * PI
        ));
        assert!(are_floats_similar(
            normalize_angle(8.0 * PI, 4.0 * PI).unwrap(),
            0.0
        ));
    }

    #[test]
    fn normalize_negative_angle() {
        assert!(are_floats_similar(
            normalize_angle(-PI, 4.0 * PI).unwrap(),
            3.0 * PI
        ));
        assert!(are_floats_similar(
            normalize_angle(-2.0 * PI, 4.0 * PI).unwrap(),
            2.0 * PI
        ));
        assert!(are_floats_similar(
            normalize_angle(-3.0 * PI, 4.0 * PI).unwrap(),
            PI
        ));
        assert!(are_floats_similar(
            normalize_angle(-4.0 * PI, 4.0 * PI).unwrap(),
            0.0
        ));
        assert!(are_floats_similar(
            normalize_angle(-5.0 * PI, 4.0 * PI).unwrap(),
            3.0 * PI
        ));
        assert!(are_floats_similar(
            normalize_angle(-6.0 * PI, 4.0 * PI).unwrap(),
            2.0 * PI
        ));
        assert!(are_floats_similar(
            normalize_angle(-7.0 * PI, 4.0 * PI).unwrap(),
            PI
        ));
        assert!(are_floats_similar(
            normalize_angle(-8.0 * PI, 4.0 * PI).unwrap(),
            0.0
        ));
    }

    #[test]
    fn normalize_edge_cases() {
        assert!(normalize_angle(f64::INFINITY, 1.0).is_err());
        assert!(normalize_angle(f64::NEG_INFINITY, 1.0).is_err());
        assert!(normalize_angle(f64::NAN, 1.0).is_err());
    }

    #[test]
    fn rationalize_zero_in_terms_of_pi() {
        assert!(rationalize_in_terms_of_pi(0.0) == Some(Ratio::new(0, 1)));
    }

    #[test]
    fn rationalize_positive_multiples_of_pi() {
        assert!(rationalize_in_terms_of_pi(PI) == Some(Ratio::new(1, 1)));
        assert!(rationalize_in_terms_of_pi(2.0 * PI) == Some(Ratio::new(2, 1)));
        assert!(rationalize_in_terms_of_pi(3.0 * PI) == Some(Ratio::new(3, 1)));
    }

    #[test]
    fn rationalize_negative_multiples_of_pi() {
        assert!(rationalize_in_terms_of_pi(-PI) == Some(Ratio::new(-1, 1)));
        assert!(rationalize_in_terms_of_pi(-2.0 * PI) == Some(Ratio::new(-2, 1)));
        assert!(rationalize_in_terms_of_pi(-3.0 * PI) == Some(Ratio::new(-3, 1)));
    }

    #[test]
    fn rationalize_non_multiples_of_pi() {
        assert!(rationalize_in_terms_of_pi(-2.5).is_none());
        assert!(rationalize_in_terms_of_pi(-1.4).is_none());
        assert!(rationalize_in_terms_of_pi(-0.3).is_none());
        assert!(rationalize_in_terms_of_pi(0.3).is_none());
        assert!(rationalize_in_terms_of_pi(1.4).is_none());
        assert!(rationalize_in_terms_of_pi(2.5).is_none());
    }

    #[test]
    fn rationalize_fractions_of_pi() {
        assert!(rationalize_in_terms_of_pi(-1.0 * PI / 2.0) == Some(Ratio::new(-1, 2)));
        assert!(rationalize_in_terms_of_pi(2.0 * PI / 3.0) == Some(Ratio::new(2, 3)));
        assert!(rationalize_in_terms_of_pi(-3.0 * PI / 4.0) == Some(Ratio::new(-3, 4)));
        assert!(rationalize_in_terms_of_pi(5.0 * PI / 6.0) == Some(Ratio::new(5, 6)));
        assert!(rationalize_in_terms_of_pi(-7.0 * PI / 8.0) == Some(Ratio::new(-7, 8)));
        assert!(rationalize_in_terms_of_pi(11.0 * PI / 12.0) == Some(Ratio::new(11, 12)));
        assert!(rationalize_in_terms_of_pi(-15.0 * PI / 16.0) == Some(Ratio::new(-15, 16)));
    }

    #[test]
    fn rationalize_big_pi_denominators() {
        assert!(rationalize_in_terms_of_pi(PI / 17.0).is_none());
        assert!(rationalize_in_terms_of_pi(PI / 18.0).is_none());
        assert!(rationalize_in_terms_of_pi(PI / 19.0).is_none());
        assert!(rationalize_in_terms_of_pi(PI / 100.0).is_none());
        assert!(rationalize_in_terms_of_pi(PI / 1000.0).is_none());
    }
}
