use super::angle_formatter::*;
use AngleFormat::*;
use PiFormat::*;

use std::f64::consts::{FRAC_PI_2, FRAC_PI_3, FRAC_PI_4, PI};

#[test]
fn format_zero_angle() {
    assert!(format(0.0, Algebra, Lowercase) == "0");
    assert!(format(-0.0, Algebra, Lowercase) == "0");
    assert!(format(1e-8, Algebra, Lowercase) == "0");
    assert!(format(-1e-8, Algebra, Lowercase) == "0");
}

#[test]
fn format_zero_angle_as_code() {
    assert!(format(0.0, Code, Lowercase) == "0");
    assert!(format(-0.0, Code, Lowercase) == "0");
    assert!(format(1e-8, Code, Lowercase) == "0");
    assert!(format(-1e-8, Code, Lowercase) == "0");
}

#[test]
fn format_pi_angles_without_denominator() {
    assert!(format(-3.0 * PI, Algebra, Lowercase) == "-3pi");
    assert!(format(-2.0 * PI, Algebra, Lowercase) == "-2pi");
    assert!(format(-PI, Algebra, Lowercase) == "-pi");
    assert!(format(PI, Algebra, Lowercase) == "pi");
    assert!(format(2.0 * PI, Algebra, Lowercase) == "2pi");
    assert!(format(3.0 * PI, Algebra, Lowercase) == "3pi");
}

#[test]
fn format_pi_angles_without_denominator_as_code() {
    assert!(format(-3.0 * PI, Code, Lowercase) == "-3 * pi");
    assert!(format(-2.0 * PI, Code, Lowercase) == "-2 * pi");
    assert!(format(-PI, Code, Lowercase) == "-pi");
    assert!(format(PI, Code, Lowercase) == "pi");
    assert!(format(2.0 * PI, Code, Lowercase) == "2 * pi");
    assert!(format(3.0 * PI, Code, Lowercase) == "3 * pi");
}

#[test]
fn format_requested_pi_format() {
    assert!(format(2.0 * PI, Algebra, Lowercase) == "2pi");
    assert!(format(2.0 * PI, Algebra, Uppercase) == "2PI");
    assert!(format(2.0 * PI, Algebra, Fancy) == "2π");
    assert!(format(2.0 * PI, Algebra, Custom { pi: "MATH_PI" }) == "2MATH_PI");
}

#[test]
fn format_requested_pi_format_as_code() {
    assert!(format(2.0 * PI, Code, Lowercase) == "2 * pi");
    assert!(format(2.0 * PI, Code, Uppercase) == "2 * PI");
    assert!(format(2.0 * PI, Code, Fancy) == "2 * π");
    assert!(format(2.0 * PI, Code, Custom { pi: "MATH_PI" }) == "2 * MATH_PI");
}

#[test]
fn format_pi_angles_without_numerator() {
    assert!(format(FRAC_PI_2, Algebra, Lowercase) == "pi/2");
    assert!(format(-FRAC_PI_2, Algebra, Lowercase) == "-pi/2");
    assert!(format(FRAC_PI_3, Algebra, Lowercase) == "pi/3");
    assert!(format(-FRAC_PI_3, Algebra, Lowercase) == "-pi/3");
    assert!(format(FRAC_PI_4, Algebra, Lowercase) == "pi/4");
    assert!(format(-FRAC_PI_4, Algebra, Lowercase) == "-pi/4");
}

#[test]
fn format_pi_angles_without_numerator_as_code() {
    assert!(format(FRAC_PI_2, Code, Lowercase) == "pi / 2");
    assert!(format(-FRAC_PI_2, Code, Lowercase) == "-pi / 2");
    assert!(format(FRAC_PI_3, Code, Lowercase) == "pi / 3");
    assert!(format(-FRAC_PI_3, Code, Lowercase) == "-pi / 3");
    assert!(format(FRAC_PI_4, Code, Lowercase) == "pi / 4");
    assert!(format(-FRAC_PI_4, Code, Lowercase) == "-pi / 4");
}

#[test]
fn format_pi_full_fraction_angles() {
    assert!(format(2.0 * FRAC_PI_3, Algebra, Lowercase) == "2pi/3");
    assert!(format(-2.0 * FRAC_PI_3, Algebra, Lowercase) == "-2pi/3");
    assert!(format(7.0 * PI / 10.0, Algebra, Lowercase) == "7pi/10");
    assert!(format(-7.0 * PI / 10.0, Algebra, Lowercase) == "-7pi/10");
}

#[test]
fn format_pi_full_fraction_angles_as_code() {
    assert!(format(2.0 * FRAC_PI_3, Code, Lowercase) == "2 * pi / 3");
    assert!(format(-2.0 * FRAC_PI_3, Code, Lowercase) == "-2 * pi / 3");
    assert!(format(7.0 * PI / 10.0, Code, Lowercase) == "7 * pi / 10");
    assert!(format(-7.0 * PI / 10.0, Code, Lowercase) == "-7 * pi / 10");
}

#[test]
fn format_non_pi_angles() {
    assert!(format(2.55, Algebra, Lowercase) == "2.55");
    assert!(format(-1.44, Algebra, Lowercase) == "-1.44");
    assert!(format(-0.33, Algebra, Lowercase) == "-0.33");
    assert!(format(0.33, Algebra, Lowercase) == "0.33");
    assert!(format(1.44, Algebra, Lowercase) == "1.44");
    assert!(format(2.55, Algebra, Lowercase) == "2.55");
}

#[test]
fn format_non_pi_angles_as_code() {
    assert!(format(2.55, Code, Lowercase) == "2.55");
    assert!(format(-1.44, Code, Lowercase) == "-1.44");
    assert!(format(-0.33, Code, Lowercase) == "-0.33");
    assert!(format(0.33, Code, Lowercase) == "0.33");
    assert!(format(1.44, Code, Lowercase) == "1.44");
    assert!(format(2.55, Code, Lowercase) == "2.55");
}

#[test]
fn format_angles_with_trailing_zeroes() {
    assert!(format(3.0, Algebra, Lowercase) == "3");
    assert!(format(-3.0, Algebra, Lowercase) == "-3");
    assert!(format(1.5, Algebra, Lowercase) == "1.5");
    assert!(format(-1.5, Algebra, Lowercase) == "-1.5");
}

#[test]
fn format_angles_with_trailing_zeroes_as_code() {
    assert!(format(3.0, Code, Lowercase) == "3");
    assert!(format(-3.0, Code, Lowercase) == "-3");
    assert!(format(1.5, Code, Lowercase) == "1.5");
    assert!(format(-1.5, Code, Lowercase) == "-1.5");
}
