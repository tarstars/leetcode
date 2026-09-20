use super::*;

fn differs_by_one_bit(left: i32, right: i32) -> bool {
    let difference = left ^ right;
    difference != 0 && difference & (difference - 1) == 0
}

/// Validate the definition instead of comparing one particular valid order.
fn check(n: i32) {
    let limit = 1usize << n as usize;
    let sequence = Solution::gray_code(n);

    assert_eq!(sequence.len(), limit, "wrong length for n = {n}");
    assert_eq!(sequence[0], 0, "the sequence must start at zero");

    let mut seen = vec![false; limit];
    for &value in &sequence {
        assert!(
            (0..limit as i32).contains(&value),
            "{value} is outside the {n}-bit range"
        );
        assert!(!seen[value as usize], "{value} appears more than once");
        seen[value as usize] = true;
    }

    for pair in sequence.windows(2) {
        assert!(
            differs_by_one_bit(pair[0], pair[1]),
            "{:b} and {:b} differ by more than one bit",
            pair[0],
            pair[1]
        );
    }
    assert!(
        differs_by_one_bit(*sequence.last().unwrap(), sequence[0]),
        "the sequence is not cyclic"
    );
}

#[test]
fn one_bit_has_the_only_possible_sequence() {
    assert_eq!(Solution::gray_code(1), vec![0, 1]);
}

#[test]
fn small_sequences_are_valid() {
    for n in 1..=6 {
        check(n);
    }
}

#[test]
fn every_value_appears_once() {
    for n in [3, 5, 8] {
        check(n);
    }
}

#[test]
fn the_maximum_input_is_valid() {
    check(16);
}
