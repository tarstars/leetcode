use super::*;

fn assert_close(actual: f64, expected: f64) {
    let tolerance = 1e-10 * expected.abs().max(1.0);
    assert!(
        (actual - expected).abs() <= tolerance,
        "expected {expected}, got {actual}"
    );
}

#[test]
fn example_1() {
    assert_close(Solution::my_pow(2.0, 10), 1024.0);
}

#[test]
fn example_2() {
    assert_close(Solution::my_pow(2.1, 3), 9.261);
}

#[test]
fn example_3() {
    assert_close(Solution::my_pow(2.0, -2), 0.25);
}

#[test]
fn exponent_zero_returns_one() {
    assert_close(Solution::my_pow(-17.5, 0), 1.0);
}

#[test]
fn zero_to_a_positive_power_is_zero() {
    assert_close(Solution::my_pow(0.0, 7), 0.0);
}

#[test]
fn negative_base_respects_exponent_parity() {
    assert_close(Solution::my_pow(-2.0, 5), -32.0);
    assert_close(Solution::my_pow(-2.0, 6), 64.0);
}

#[test]
fn negative_exponent_takes_the_reciprocal() {
    assert_close(Solution::my_pow(0.5, -3), 8.0);
}

#[test]
fn handles_the_minimum_i32_exponent() {
    assert_close(Solution::my_pow(1.0, i32::MIN), 1.0);
}
