use super::angle_formatter::*;
use PiFormat::*;

use std::f64::consts::{FRAC_PI_2, FRAC_PI_3, FRAC_PI_4, PI};

#[test]
fn format_zero_angle() {
    assert!(format(0.0, Lowercase) == "0");
    assert!(format(-0.0, Lowercase) == "0");
    assert!(format(1e-8, Lowercase) == "0");
    assert!(format(-1e-8, Lowercase) == "0");
}

#[test]
fn format_pi_angles_without_denominator() {
    assert!(format(-3.0 * PI, Lowercase) == "-3pi");
    assert!(format(-2.0 * PI, Lowercase) == "-2pi");
    assert!(format(-PI, Lowercase) == "-pi");
    assert!(format(PI, Lowercase) == "pi");
    assert!(format(2.0 * PI, Lowercase) == "2pi");
    assert!(format(3.0 * PI, Lowercase) == "3pi");
}

#[test]
fn format_requested_pi_format() {
    assert!(format(2.0 * PI, Lowercase) == "2pi");
    assert!(format(2.0 * PI, Uppercase) == "2PI");
    assert!(format(2.0 * PI, Pretty) == "2π");
}

#[test]
fn format_pi_angles_without_numerator() {
    assert!(format(FRAC_PI_2, Lowercase) == "pi/2");
    assert!(format(-FRAC_PI_2, Lowercase) == "-pi/2");
    assert!(format(FRAC_PI_3, Lowercase) == "pi/3");
    assert!(format(-FRAC_PI_3, Lowercase) == "-pi/3");
    assert!(format(FRAC_PI_4, Lowercase) == "pi/4");
    assert!(format(-FRAC_PI_4, Lowercase) == "-pi/4");
}

#[test]
fn format_pi_full_fraction_angles() {
    assert!(format(2.0 * FRAC_PI_3, Lowercase) == "2pi/3");
    assert!(format(-2.0 * FRAC_PI_3, Lowercase) == "-2pi/3");
    assert!(format(7.0 * PI / 10.0, Lowercase) == "7pi/10");
    assert!(format(-7.0 * PI / 10.0, Lowercase) == "-7pi/10");
}

#[test]
fn format_non_pi_angles() {
    assert!(format(2.55, Lowercase) == "2.55");
    assert!(format(-1.44, Lowercase) == "-1.44");
    assert!(format(-0.33, Lowercase) == "-0.33");
    assert!(format(0.33, Lowercase) == "0.33");
    assert!(format(1.44, Lowercase) == "1.44");
    assert!(format(2.55, Lowercase) == "2.55");
}

#[test]
fn format_angles_with_trailing_zeroes() {
    assert!(format(3.0, Lowercase) == "3");
    assert!(format(-3.0, Lowercase) == "-3");
    assert!(format(1.5, Lowercase) == "1.5");
    assert!(format(-1.5, Lowercase) == "-1.5");
}
