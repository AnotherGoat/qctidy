use super::math::{are_floats_similar, rationalize_in_terms_of_pi};

/// Return a nicely formatted string for the provided angle, where multiples of pi are used whenever possible.
pub(crate) fn format_angle(angle: f64) -> String {
    if are_floats_similar(angle, 0.0) {
        return "0".to_string();
    }

    if let Some(fraction) = rationalize_in_terms_of_pi(angle) {
        let numerator = *fraction.numer();
        let denominator = *fraction.denom();

        if numerator == denominator {
            return "π".to_string();
        }
        if numerator == -denominator {
            return "-π".to_string();
        }
        if numerator == 1 {
            return format!("π/{}", denominator);
        }
        if numerator == -1 {
            return format!("-π/{}", denominator);
        }
        if denominator == 1 {
            return format!("{}π", numerator);
        }

        return format!("{}π/{}", numerator, denominator);
    }

    trim_trailing_zeroes(&format!("{:.2}", angle))
}

fn trim_trailing_zeroes(angle: &str) -> String {
    angle
        .trim_end_matches('0')
        .trim_end_matches('.')
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::f64::consts::PI;

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
        assert!(format_angle(PI / 2.0) == "π/2");
        assert!(format_angle(-PI / 2.0) == "-π/2");
        assert!(format_angle(PI / 3.0) == "π/3");
        assert!(format_angle(-PI / 3.0) == "-π/3");
        assert!(format_angle(PI / 4.0) == "π/4");
        assert!(format_angle(-PI / 4.0) == "-π/4");
    }

    #[test]
    fn format_pi_full_fraction_angles() {
        assert!(format_angle(2.0 * PI / 3.0) == "2π/3");
        assert!(format_angle(-2.0 * PI / 3.0) == "-2π/3");
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
}
