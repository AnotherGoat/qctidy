use crate::domain::number::truncate_f64_to_i64;

#[test]
fn truncate_integer_f64_to_i64() {
    assert!(truncate_f64_to_i64(2.0) == 2);
    assert!(truncate_f64_to_i64(1.0) == 1);
    assert!(truncate_f64_to_i64(0.0) == 0);
    assert!(truncate_f64_to_i64(-1.0) == -1);
    assert!(truncate_f64_to_i64(-2.0) == -2);
}

#[test]
fn truncate_f64_to_i64_is_rounded() {
    assert!(truncate_f64_to_i64(0.1) == 0);
    assert!(truncate_f64_to_i64(0.5) == 1);
    assert!(truncate_f64_to_i64(0.9) == 1);
    assert!(truncate_f64_to_i64(1.1) == 1);
    assert!(truncate_f64_to_i64(1.5) == 2);
    assert!(truncate_f64_to_i64(1.9) == 2);
}
