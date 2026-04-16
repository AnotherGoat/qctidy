use num_rational::Ratio;

use super::math::*;

use std::f64::consts::{FRAC_PI_2, FRAC_PI_3, FRAC_PI_4, FRAC_PI_6, FRAC_PI_8, PI};

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
    assert!(rationalize_in_terms_of_pi(-1.0 * FRAC_PI_2) == Some(Ratio::new(-1, 2)));
    assert!(rationalize_in_terms_of_pi(2.0 * FRAC_PI_3) == Some(Ratio::new(2, 3)));
    assert!(rationalize_in_terms_of_pi(-3.0 * FRAC_PI_4) == Some(Ratio::new(-3, 4)));
    assert!(rationalize_in_terms_of_pi(5.0 * FRAC_PI_6) == Some(Ratio::new(5, 6)));
    assert!(rationalize_in_terms_of_pi(-7.0 * FRAC_PI_8) == Some(Ratio::new(-7, 8)));
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
