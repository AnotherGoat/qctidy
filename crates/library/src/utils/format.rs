use super::math::{are_floats_similar, rationalize_in_terms_of_pi};

const PI: &str = "π";

/// Return a nicely formatted string for the provided angle, where multiples of pi are used whenever possible.
pub(crate) fn format_angle(angle: f64) -> String {
    if are_floats_similar(angle, 0.0) {
        return "0".to_string();
    }

    if let Some(fraction) = rationalize_in_terms_of_pi(angle) {
        let numerator = *fraction.numer();
        let denominator = *fraction.denom();

        if numerator == denominator {
            return PI.to_string();
        }

        if numerator == -denominator {
            return format!("-{}", PI);
        }

        if numerator == 1 {
            return format!("{}/{}", PI, denominator);
        }

        if numerator == -1 {
            return format!("-{}/{}", PI, denominator);
        }

        if denominator == 1 {
            return format!("{}{}", numerator, PI);
        }

        return format!("{}{}/{}", numerator, PI, denominator);
    }

    trim_trailing_zeroes(&format!("{:.2}", angle))
}

fn trim_trailing_zeroes(angle: &str) -> String {
    angle
        .trim_end_matches('0')
        .trim_end_matches('.')
        .to_string()
}
