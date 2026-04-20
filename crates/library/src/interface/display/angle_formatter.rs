use crate::domain::math;

/// The format to used to display the pi constant.
#[must_use]
pub(crate) enum PiFormat {
    /// Display the pi constant as `pi`, ASCII-compatible.
    Lowercase,
    /// Display the pi constant as `PI`, ASCII-compatible.
    Uppercase,
    /// Display the pi constant as `π`.
    Pretty,
}

impl PiFormat {
    fn to_string(&self) -> &str {
        use PiFormat::*;

        match *self {
            Lowercase => "pi",
            Uppercase => "PI",
            Pretty => "π",
        }
    }
}

/// Return a formatted string for the provided angle, where multiples of pi are used whenever possible.
///
/// The output depends on the requested pi format.
pub(crate) fn format(angle: f64, pi_format: PiFormat) -> String {
    if math::are_floats_similar(angle, 0.0) {
        return "0".to_owned();
    }

    if let Some(fraction) = math::rationalize_in_terms_of_pi(angle) {
        let numerator = *fraction.numer();
        let denominator = *fraction.denom();
        let pi = pi_format.to_string();

        if numerator == denominator {
            return pi.to_owned();
        }

        if numerator == -denominator {
            return format!("-{pi}");
        }

        if numerator == 1 {
            return format!("{pi}/{denominator}");
        }

        if numerator == -1 {
            return format!("-{pi}/{denominator}");
        }

        if denominator == 1 {
            return format!("{numerator}{pi}");
        }

        return format!("{numerator}{pi}/{denominator}");
    }

    trim_trailing_zeroes(&format!("{angle:.2}"))
}

fn trim_trailing_zeroes(angle: &str) -> String {
    angle.trim_end_matches('0').trim_end_matches('.').to_owned()
}
