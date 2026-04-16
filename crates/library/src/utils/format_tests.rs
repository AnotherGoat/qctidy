use super::format::*;

use std::f64::consts::{FRAC_PI_2, FRAC_PI_3, FRAC_PI_4, PI};

#[test]
fn format_zero_angle() {
    assert!(format_angle(0.0) == "0");
    assert!(format_angle(-0.0) == "0");
    assert!(format_angle(1e-8) == "0");
    assert!(format_angle(-1e-8) == "0");
}

#[test]
fn format_pi_angles_without_denominator() {
    assert!(format_angle(-3.0 * PI) == "-3π");
    assert!(format_angle(-2.0 * PI) == "-2π");
    assert!(format_angle(-PI) == "-π");
    assert!(format_angle(PI) == "π");
    assert!(format_angle(2.0 * PI) == "2π");
    assert!(format_angle(3.0 * PI) == "3π");
}

#[test]
fn format_pi_angles_without_numerator() {
    assert!(format_angle(FRAC_PI_2) == "π/2");
    assert!(format_angle(-FRAC_PI_2) == "-π/2");
    assert!(format_angle(FRAC_PI_3) == "π/3");
    assert!(format_angle(-FRAC_PI_3) == "-π/3");
    assert!(format_angle(FRAC_PI_4) == "π/4");
    assert!(format_angle(-FRAC_PI_4) == "-π/4");
}

#[test]
fn format_pi_full_fraction_angles() {
    assert!(format_angle(2.0 * FRAC_PI_3) == "2π/3");
    assert!(format_angle(-2.0 * FRAC_PI_3) == "-2π/3");
    assert!(format_angle(7.0 * PI / 10.0) == "7π/10");
    assert!(format_angle(-7.0 * PI / 10.0) == "-7π/10");
}

#[test]
fn format_non_pi_angles() {
    assert!(format_angle(2.55) == "2.55");
    assert!(format_angle(-1.44) == "-1.44");
    assert!(format_angle(-0.33) == "-0.33");
    assert!(format_angle(0.33) == "0.33");
    assert!(format_angle(1.44) == "1.44");
    assert!(format_angle(2.55) == "2.55");
}

#[test]
fn format_angles_with_trailing_zeroes() {
    assert!(format_angle(3.0) == "3");
    assert!(format_angle(-3.0) == "-3");
    assert!(format_angle(1.5) == "1.5");
    assert!(format_angle(-1.5) == "-1.5");
}
