use faer::{complex::Complex64, mat};
use num_rational::Ratio;

use crate::domain::math::{
    are_floats_equal, are_matrices_equal, are_option_floats_equal, normalize_angle,
    rationalize_in_terms_of_pi,
};

use std::f64::consts::{FRAC_PI_2, FRAC_PI_3, FRAC_PI_4, FRAC_PI_6, FRAC_PI_8, PI};

#[test]
fn are_same_floats_equal() {
    assert!(are_floats_equal(5.0, 5.0));
    assert!(are_floats_equal(PI, PI));
    assert!(are_floats_equal(FRAC_PI_2, FRAC_PI_2));
}

#[test]
fn are_different_floats_equal() {
    assert!(!are_floats_equal(5.0, 10.0));
    assert!(!are_floats_equal(-3.0, 3.0));
}

#[test]
fn are_infinities_equal() {
    assert!(are_floats_equal(f64::INFINITY, f64::INFINITY));
    assert!(are_floats_equal(f64::NEG_INFINITY, f64::NEG_INFINITY));
}

#[test]
fn are_nan_floats_equal() {
    assert!(!are_floats_equal(f64::NAN, f64::NAN));
}

#[test]
fn are_close_floats_equal() {
    assert!(!are_floats_equal(0.0, 1.0));
    assert!(!are_floats_equal(0.0, 1e-1));
    assert!(!are_floats_equal(0.0, 1e-2));
    assert!(!are_floats_equal(0.0, 1e-3));
    assert!(!are_floats_equal(0.0, 1e-4));
    assert!(!are_floats_equal(0.0, 1e-5));
    assert!(!are_floats_equal(0.0, 1e-6));
    assert!(!are_floats_equal(0.0, 1e-7));
}

#[test]
#[expect(clippy::approx_constant)]
fn are_very_close_floats_equal() {
    assert!(are_floats_equal(0.0, 1e-8));
    assert!(are_floats_equal(0.3, 0.300_000_01));
    assert!(are_floats_equal(3.14159, PI));
}

#[test]
fn are_some_floats_equal() {
    assert!(are_option_floats_equal(Some(5.0_f64), Some(5.0_f64)));
    assert!(are_option_floats_equal(Some(PI), Some(PI)));
    assert!(are_option_floats_equal(Some(FRAC_PI_2), Some(FRAC_PI_2)));
}

#[test]
fn are_none_floats_equal() {
    assert!(are_option_floats_equal(None, None));
}

#[test]
fn are_none_and_some_floats_not_equal() {
    assert!(!are_option_floats_equal(None, Some(3.0_f64)));
    assert!(!are_option_floats_equal(Some(3.0_f64), None));
}

#[test]
fn are_same_matrices_equal() {
    let matrix = mat![
        [Complex64::from(1.0_f64), Complex64::new(2.0_f64, 3.0_f64)],
        [Complex64::from(4.0_f64), Complex64::new(5.0_f64, 6.0_f64)]
    ];

    assert!(are_matrices_equal(&matrix, &matrix.clone()));
}

#[test]
fn matrices_with_different_row_count_are_not_equal() {
    let one_row = mat![[Complex64::from(1.0_f64)]];

    let two_rows = mat![[Complex64::from(1.0_f64)], [Complex64::from(2.0_f64)]];

    assert!(!are_matrices_equal(&one_row, &two_rows));
}

#[test]
fn matrices_with_different_column_count_are_not_equal() {
    let one_column = mat![[Complex64::from(1.0_f64)]];

    let two_columns = mat![[Complex64::from(1.0_f64), Complex64::from(2.0_f64)]];

    assert!(!are_matrices_equal(&one_column, &two_columns));
}

#[test]
fn close_matrices_are_not_equal() {
    let first = mat![[Complex64::from(0.0_f64)],];

    let second = mat![[Complex64::from(1e-7_f64)],];

    assert!(!are_matrices_equal(&first, &second));
}

#[test]
fn very_close_matrices_are_equal() {
    let first = mat![[Complex64::from(0.0_f64)],];

    let second = mat![[Complex64::from(1e-8_f64)],];

    assert!(are_matrices_equal(&first, &second));
}

#[test]
fn normalize_zero_angle() {
    assert!(are_floats_equal(
        normalize_angle(0.0, 4.0 * PI).unwrap(),
        0.0
    ));
    assert!(are_floats_equal(
        normalize_angle(-0.0, 4.0 * PI).unwrap(),
        0.0
    ));
}

#[test]
fn normalize_positive_angle() {
    assert!(are_floats_equal(normalize_angle(PI, 4.0 * PI).unwrap(), PI));
    assert!(are_floats_equal(
        normalize_angle(2.0 * PI, 4.0 * PI).unwrap(),
        2.0 * PI
    ));
    assert!(are_floats_equal(
        normalize_angle(3.0 * PI, 4.0 * PI).unwrap(),
        3.0 * PI
    ));
    assert!(are_floats_equal(
        normalize_angle(4.0 * PI, 4.0 * PI).unwrap(),
        0.0
    ));
    assert!(are_floats_equal(
        normalize_angle(5.0 * PI, 4.0 * PI).unwrap(),
        PI
    ));
    assert!(are_floats_equal(
        normalize_angle(6.0 * PI, 4.0 * PI).unwrap(),
        2.0 * PI
    ));
    assert!(are_floats_equal(
        normalize_angle(7.0 * PI, 4.0 * PI).unwrap(),
        3.0 * PI
    ));
    assert!(are_floats_equal(
        normalize_angle(8.0 * PI, 4.0 * PI).unwrap(),
        0.0
    ));
}

#[test]
fn normalize_negative_angle() {
    assert!(are_floats_equal(
        normalize_angle(-PI, 4.0 * PI).unwrap(),
        3.0 * PI
    ));
    assert!(are_floats_equal(
        normalize_angle(-2.0 * PI, 4.0 * PI).unwrap(),
        2.0 * PI
    ));
    assert!(are_floats_equal(
        normalize_angle(-3.0 * PI, 4.0 * PI).unwrap(),
        PI
    ));
    assert!(are_floats_equal(
        normalize_angle(-4.0 * PI, 4.0 * PI).unwrap(),
        0.0
    ));
    assert!(are_floats_equal(
        normalize_angle(-5.0 * PI, 4.0 * PI).unwrap(),
        3.0 * PI
    ));
    assert!(are_floats_equal(
        normalize_angle(-6.0 * PI, 4.0 * PI).unwrap(),
        2.0 * PI
    ));
    assert!(are_floats_equal(
        normalize_angle(-7.0 * PI, 4.0 * PI).unwrap(),
        PI
    ));
    assert!(are_floats_equal(
        normalize_angle(-8.0 * PI, 4.0 * PI).unwrap(),
        0.0
    ));
}

#[test]
fn normalize_edge_cases() {
    normalize_angle(f64::INFINITY, 1.0).unwrap_err();
    normalize_angle(f64::NEG_INFINITY, 1.0).unwrap_err();
    normalize_angle(f64::NAN, 1.0).unwrap_err();
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
    assert!(rationalize_in_terms_of_pi(-FRAC_PI_2) == Some(Ratio::new(-1, 2)));
    assert!(rationalize_in_terms_of_pi(2.0 * FRAC_PI_3) == Some(Ratio::new(2, 3)));
    assert!(rationalize_in_terms_of_pi(-3.0 * FRAC_PI_4) == Some(Ratio::new(-3, 4)));
    assert!(rationalize_in_terms_of_pi(5.0 * FRAC_PI_6) == Some(Ratio::new(5, 6)));
    assert!(rationalize_in_terms_of_pi(-7.0 * FRAC_PI_8) == Some(Ratio::new(-7, 8)));
    assert!(rationalize_in_terms_of_pi(11.0 * PI / 12.0) == Some(Ratio::new(11, 12)));
    assert!(rationalize_in_terms_of_pi(-15.0 * PI / 16.0) == Some(Ratio::new(-15, 16)));
}

#[test]
fn rationalize_pi_constant_fractions() {
    assert!(rationalize_in_terms_of_pi(FRAC_PI_2) == Some(Ratio::new(1, 2)));
    assert!(rationalize_in_terms_of_pi(FRAC_PI_3) == Some(Ratio::new(1, 3)));
    assert!(rationalize_in_terms_of_pi(FRAC_PI_4) == Some(Ratio::new(1, 4)));
    assert!(rationalize_in_terms_of_pi(FRAC_PI_6) == Some(Ratio::new(1, 6)));
    assert!(rationalize_in_terms_of_pi(FRAC_PI_8) == Some(Ratio::new(1, 8)));
}

#[test]
fn rationalize_big_pi_denominators() {
    assert!(rationalize_in_terms_of_pi(PI / 17.0).is_none());
    assert!(rationalize_in_terms_of_pi(PI / 18.0).is_none());
    assert!(rationalize_in_terms_of_pi(PI / 19.0).is_none());
    assert!(rationalize_in_terms_of_pi(PI / 100.0).is_none());
    assert!(rationalize_in_terms_of_pi(PI / 1000.0).is_none());
}

#[test]
fn rationalize_infinity_in_terms_of_pi() {
    assert!(rationalize_in_terms_of_pi(f64::INFINITY).is_none());
    assert!(rationalize_in_terms_of_pi(f64::NEG_INFINITY).is_none());
}

#[test]
fn rationalize_nan_in_terms_of_pi() {
    assert!(rationalize_in_terms_of_pi(f64::NAN).is_none());
}
