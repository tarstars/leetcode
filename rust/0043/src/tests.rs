use super::*;

fn multiply(num1: &str, num2: &str) -> String {
    Solution::multiply(num1.to_owned(), num2.to_owned())
}

#[test]
fn example_1() {
    assert_eq!(multiply("2", "3"), "6");
}

#[test]
fn example_2() {
    assert_eq!(multiply("123", "456"), "56088");
}

#[test]
fn multiplication_by_zero_returns_canonical_zero() {
    assert_eq!(multiply("0", "9133"), "0");
    assert_eq!(multiply("4817", "0"), "0");
    assert_eq!(multiply("0", "0"), "0");
}

#[test]
fn multiplication_by_one_preserves_the_other_number() {
    assert_eq!(multiply("1", "9876543210123456789"), "9876543210123456789");
}

#[test]
fn propagates_carries_across_digits() {
    assert_eq!(multiply("99", "99"), "9801");
    assert_eq!(multiply("999", "9"), "8991");
}

#[test]
fn multiplies_larger_values() {
    assert_eq!(multiply("123456789", "987654321"), "121932631112635269");
}

#[test]
fn handles_maximum_length_inputs() {
    let num1 = format!("1{}", "0".repeat(199));
    let num2 = format!("1{}", "0".repeat(199));
    let expected = format!("1{}", "0".repeat(398));
    assert_eq!(multiply(&num1, &num2), expected);
}
