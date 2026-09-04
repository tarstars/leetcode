use super::*;

/// Incrementing without any carry loop: find the rightmost digit that is not a
/// nine, bump it, and zero everything to its right. All nines is the one case
/// with no such digit, and it is exactly the case that grows the array. This is
/// a different formulation from right-to-left carry propagation, so the two are
/// unlikely to be wrong in the same way.
fn reference(digits: &[i32]) -> Vec<i32> {
    match digits.iter().rposition(|&d| d != 9) {
        Some(last) => {
            let mut result = digits.to_vec();
            result[last] += 1;
            result[last + 1..].fill(0);
            result
        }
        None => {
            let mut result = vec![0; digits.len() + 1];
            result[0] = 1;
            result
        }
    }
}

fn plus_one(digits: &[i32]) -> Vec<i32> {
    Solution::plus_one(digits.to_vec())
}

/// Every digit array of the given length that the constraints permit: no
/// leading zero, unless the whole number is the single digit 0.
fn every_legal_input(len: usize) -> impl Iterator<Item = Vec<i32>> {
    let total = 10u32.pow(len as u32);

    (0..total).filter_map(move |mut code| {
        let mut digits = vec![0; len];
        for slot in digits.iter_mut().rev() {
            *slot = (code % 10) as i32;
            code /= 10;
        }

        let legal = digits[0] != 0 || len == 1;
        legal.then_some(digits)
    })
}

#[test]
fn example_1() {
    assert_eq!(plus_one(&[1, 2, 3]), vec![1, 2, 4]);
}

#[test]
fn example_2() {
    assert_eq!(plus_one(&[4, 3, 2, 1]), vec![4, 3, 2, 2]);
}

/// A single nine is the smallest input that grows the array.
#[test]
fn example_3() {
    assert_eq!(plus_one(&[9]), vec![1, 0]);
}

/// Zero is a legal input despite the no-leading-zeros rule, being one digit.
#[test]
fn single_digits() {
    for d in 0..=8 {
        assert_eq!(plus_one(&[d]), vec![d + 1], "{d} + 1");
    }
    assert_eq!(plus_one(&[9]), vec![1, 0]);
}

/// No carry at all: only the last digit changes.
#[test]
fn no_carry() {
    assert_eq!(plus_one(&[1, 2, 3, 4]), vec![1, 2, 3, 5]);
    assert_eq!(plus_one(&[5, 0, 0]), vec![5, 0, 1]);
}

/// The carry stops partway through the array.
#[test]
fn carry_stops_partway() {
    assert_eq!(plus_one(&[1, 9]), vec![2, 0]);
    assert_eq!(plus_one(&[1, 9, 9]), vec![2, 0, 0]);
    assert_eq!(plus_one(&[1, 2, 9, 9]), vec![1, 3, 0, 0]);
    assert_eq!(plus_one(&[8, 9, 9, 9]), vec![9, 0, 0, 0]);
}

/// All nines is the only shape that lengthens the array.
#[test]
fn all_nines_grows_the_array() {
    for len in 1..=20usize {
        let expected: Vec<i32> = std::iter::once(1)
            .chain(std::iter::repeat(0).take(len))
            .collect();
        assert_eq!(plus_one(&vec![9; len]), expected, "{len} nines");
    }
}

/// Interior zeros must survive untouched when the carry stops before them.
#[test]
fn interior_zeros_are_preserved() {
    assert_eq!(plus_one(&[1, 0, 0, 0]), vec![1, 0, 0, 1]);
    assert_eq!(plus_one(&[1, 0, 0, 9]), vec![1, 0, 1, 0]);
    assert_eq!(plus_one(&[9, 0, 9, 9]), vec![9, 1, 0, 0]);
}

/// The reference must agree with real arithmetic before it is trusted as an
/// oracle for the exhaustive sweeps: for inputs short enough to fit, parse the
/// digits into a u128, add one, and read the result back out.
#[test]
fn the_reference_agrees_with_integer_arithmetic() {
    for len in 1..=5usize {
        for digits in every_legal_input(len) {
            let value: u128 = digits.iter().fold(0, |acc, &d| acc * 10 + d as u128);
            let expected: Vec<i32> = (value + 1)
                .to_string()
                .bytes()
                .map(|b| (b - b'0') as i32)
                .collect();

            assert_eq!(reference(&digits), expected, "{digits:?}");
        }
    }
}

/// Exhaustive: every legal input up to five digits.
#[test]
fn matches_reference_on_every_short_input() {
    for len in 1..=5usize {
        for digits in every_legal_input(len) {
            assert_eq!(plus_one(&digits), reference(&digits), "{digits:?}");
        }
    }
}

/// The output must itself be a legal digit array: single digits only, and no
/// leading zero.
#[test]
fn the_result_is_a_well_formed_digit_array() {
    for len in 1..=4usize {
        for digits in every_legal_input(len) {
            let result = plus_one(&digits);

            assert!(!result.is_empty(), "{digits:?} produced an empty array");
            assert!(
                result.iter().all(|&d| (0..=9).contains(&d)),
                "{digits:?} -> {result:?} has a non-digit"
            );
            assert!(
                result[0] != 0 || result.len() == 1,
                "{digits:?} -> {result:?} has a leading zero"
            );
        }
    }
}

/// 100 digits is the constraint's maximum, and 100 nines is far beyond every
/// built-in integer type — u128 tops out at 39 digits. Converting the array to
/// a number, incrementing, and converting back cannot work here.
#[test]
fn longest_allowed_input() {
    let expected: Vec<i32> = std::iter::once(1)
        .chain(std::iter::repeat(0).take(100))
        .collect();
    assert_eq!(plus_one(&vec![9; 100]), expected);

    let mut digits = vec![9; 100];
    digits[0] = 1;
    let mut expected = vec![0; 100];
    expected[0] = 2;
    assert_eq!(plus_one(&digits), expected);

    let mut digits = vec![0; 100];
    digits[0] = 7;
    let mut expected = digits.clone();
    expected[99] = 1;
    assert_eq!(plus_one(&digits), expected);
}

/// Long inputs where the carry runs a controlled distance, checked against the
/// reference at lengths no integer type could hold.
#[test]
fn long_inputs_with_partial_carries() {
    for len in [40usize, 64, 99, 100] {
        for trailing_nines in [0usize, 1, 7, len - 1] {
            let mut digits = vec![1; len];
            digits[len - trailing_nines..].fill(9);
            assert_eq!(
                plus_one(&digits),
                reference(&digits),
                "{len} digits with {trailing_nines} trailing nines"
            );
        }
    }
}
