use std::fmt;

use crate::{display::number_formatter, domain::math};

/// Format to used to display operations used when displaying angles.
#[derive(Debug, Clone, Copy)]
#[must_use]
pub enum AngleFormat {
    /// Display the angle in an algebraic way (no spaces, no multiplication symbols).
    Algebra,
    /// Display the angle in as a operation that could be used un source code.
    Code,
}

/// Format to used to display the pi constant in angles.
#[derive(Debug, Clone, Copy)]
#[must_use]
pub enum PiFormat {
    /// Display the pi constant as `pi`, ASCII-compatible.
    Lowercase,
    /// Display the pi constant as `PI`, ASCII-compatible.
    Uppercase,
    /// Display the pi constant as `π`.
    Fancy,
    /// Display the pi constant using a custom string.
    Custom { pi: &'static str },
}

impl fmt::Display for PiFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use PiFormat::*;

        let name = match *self {
            Lowercase => "pi",
            Uppercase => "PI",
            Fancy => "π",
            Custom { pi } => pi,
        };

        f.write_str(name)
    }
}

/// Return a formatted string for the provided angle, where multiples of pi are used whenever possible.
///
/// The output depends on the requested pi format.
#[must_use]
pub fn format(angle: f64, angle_format: AngleFormat, pi_format: PiFormat) -> String {
    use AngleFormat::*;

    if math::are_floats_equal(angle, 0.0) {
        return "0".to_owned();
    }

    if let Some(fraction) = math::rationalize_in_terms_of_pi(angle) {
        let numerator = *fraction.numer();
        let denominator = *fraction.denom();
        let pi = pi_format.to_string();

        let (multiplication, division) = match angle_format {
            Algebra => ("", "/"),
            Code => (" * ", " / "),
        };

        if numerator == denominator {
            return pi;
        }

        if numerator == -denominator {
            return format!("-{pi}");
        }

        if numerator == 1 {
            return format!("{pi}{division}{denominator}");
        }

        if numerator == -1 {
            return format!("-{pi}{division}{denominator}");
        }

        if denominator == 1 {
            return format!("{numerator}{multiplication}{pi}");
        }

        return format!("{numerator}{multiplication}{pi}{division}{denominator}");
    }

    number_formatter::format(angle)
}
