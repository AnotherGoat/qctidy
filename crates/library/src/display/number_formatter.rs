use faer::complex::Complex64;

use crate::domain::math;

/// Format a number by rounding it to 2 decimal places and then removing trailing zeroes.
#[must_use]
pub(crate) fn format(number: f64) -> String {
    trim_trailing_zeroes(&format!("{number:.2}"))
}

fn trim_trailing_zeroes(number: &str) -> String {
    number
        .trim_end_matches('0')
        .trim_end_matches('.')
        .to_owned()
}

#[must_use]
pub(crate) fn format_complex(complex: Complex64) -> String {
    let real = if complex.re.abs() < math::EPSILON {
        0.0_f64
    } else {
        complex.re
    };

    let imaginary = if complex.im.abs() < math::EPSILON {
        0.0_f64
    } else {
        complex.im
    };

    match (real, imaginary) {
        (0.0, 0.0) => "0".to_owned(),
        (_, 0.0) => format(real),
        (0.0, _) => format(imaginary),
        _ => {
            let sign = if imaginary.is_sign_positive() {
                "+"
            } else {
                "-"
            };

            format!("{} {sign} {}i", format(real), format(imaginary.abs()))
        }
    }
}
