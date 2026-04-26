use crate::{domain::math, interface::display::number_formatter};

/// The format to used to display the pi constant.
#[derive(Debug, Clone, Copy)]
#[must_use]
pub enum PiFormat {
    /// Display the pi constant as `pi`, ASCII-compatible.
    Lowercase,
    /// Display the pi constant as `PI`, ASCII-compatible.
    Uppercase,
    /// Display the pi constant as `π`.
    Fancy,
}

impl PiFormat {
    fn to_string(&self) -> &str {
        use PiFormat::*;

        match *self {
            Lowercase => "pi",
            Uppercase => "PI",
            Fancy => "π",
        }
    }
}

/// Return a formatted string for the provided angle, where multiples of pi are used whenever possible.
///
/// The output depends on the requested pi format.
pub(crate) fn format(angle: f64, pi_format: PiFormat) -> String {
    if math::are_floats_equal(angle, 0.0) {
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

    number_formatter::format(angle)
}
