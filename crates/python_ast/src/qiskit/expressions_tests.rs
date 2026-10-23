use std::f64::consts::PI;

use super::expressions::{evaluate_integer, evaluate_number};

#[test]
fn parses_integers() {
    assert_eq!(evaluate_integer("0"), Some(0));
    assert_eq!(evaluate_integer(" 42 "), Some(42));
    assert_eq!(evaluate_integer(""), None);
    assert_eq!(evaluate_integer("3.0"), None);
    assert_eq!(evaluate_integer("n"), None);
}

#[test]
fn resolves_pi_constants() {
    assert!((evaluate_number("numpy.pi / 4").unwrap() - PI / 4.0).abs() < 1e-12);
    assert!((evaluate_number("2 * math.pi / 3").unwrap() - 2.0 * PI / 3.0).abs() < 1e-12);
    assert!((evaluate_number("-np.pi / 2").unwrap() + PI / 2.0).abs() < 1e-12);
    assert!((evaluate_number("pi").unwrap() - PI).abs() < 1e-12);
}

#[test]
fn evaluates_arithmetic() {
    assert!((evaluate_number("1 / 2").unwrap() - 0.5).abs() < 1e-12);
    assert!((evaluate_number("2**3").unwrap() - 8.0).abs() < 1e-12);
    assert!((evaluate_number("1e-3").unwrap() - 0.001).abs() < 1e-15);
    assert!((evaluate_number("2 * (3 + 4)").unwrap() - 14.0).abs() < 1e-12);
}

#[test]
fn rejects_unresolvable_expressions() {
    assert_eq!(evaluate_number("x"), None);
    assert_eq!(evaluate_number("circuit.q(0)"), None);
    assert_eq!(evaluate_number("2e"), None);
    assert_eq!(evaluate_number("1 +"), None);
}
