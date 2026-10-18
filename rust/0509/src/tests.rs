use super::*;

#[test]
fn examples() {
    assert_eq!(Solution::fib(2), 1);
    assert_eq!(Solution::fib(3), 2);
    assert_eq!(Solution::fib(4), 3);
}

#[test]
fn base_cases() {
    assert_eq!(Solution::fib(0), 0);
    assert_eq!(Solution::fib(1), 1);
}

#[test]
fn a_larger_number() {
    assert_eq!(Solution::fib(10), 55);
}

#[test]
fn largest_input() {
    assert_eq!(Solution::fib(30), 832_040);
}
