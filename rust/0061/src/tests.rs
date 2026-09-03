use super::*;

fn rotate(values: &[i32], k: i32) -> Vec<i32> {
    to_values(&Solution::rotate_right(from_values(values), k))
}

/// The reference implementation the property tests compare against: rotating
/// right by k moves the last `k % len` values to the front.
fn expected(values: &[i32], k: i32) -> Vec<i32> {
    if values.is_empty() {
        return Vec::new();
    }

    let split = values.len() - (k as usize) % values.len();
    let mut rotated = values[split..].to_vec();
    rotated.extend_from_slice(&values[..split]);
    rotated
}

#[test]
fn example_1() {
    assert_eq!(rotate(&[1, 2, 3, 4, 5], 2), vec![4, 5, 1, 2, 3]);
}

/// k exceeds the length here, so it has to wrap.
#[test]
fn example_2() {
    assert_eq!(rotate(&[0, 1, 2], 4), vec![2, 0, 1]);
}

/// The constraints allow zero nodes, so `head` can be None.
#[test]
fn empty_list() {
    assert_eq!(rotate(&[], 0), Vec::<i32>::new());
    assert_eq!(rotate(&[], 7), Vec::<i32>::new());
}

#[test]
fn single_node() {
    assert_eq!(rotate(&[7], 0), vec![7]);
    assert_eq!(rotate(&[7], 1), vec![7]);
    assert_eq!(rotate(&[7], 2_000_000_000), vec![7]);
}

#[test]
fn k_of_zero_is_the_identity() {
    assert_eq!(rotate(&[1, 2, 3, 4, 5], 0), vec![1, 2, 3, 4, 5]);
}

/// A full turn, and any whole number of turns, leaves the list unchanged.
#[test]
fn whole_turns_are_the_identity() {
    assert_eq!(rotate(&[1, 2, 3, 4], 4), vec![1, 2, 3, 4]);
    assert_eq!(rotate(&[1, 2, 3, 4], 8), vec![1, 2, 3, 4]);
    assert_eq!(rotate(&[1, 2, 3, 4], 400), vec![1, 2, 3, 4]);
}

/// Rotating by len - 1 is the same as rotating left by one.
#[test]
fn rotating_by_one_less_than_the_length() {
    assert_eq!(rotate(&[1, 2, 3, 4, 5], 4), vec![2, 3, 4, 5, 1]);
}

/// Equal values can't reveal the reordering, but the length must survive:
/// dropped or duplicated nodes show up here.
#[test]
fn repeated_values() {
    assert_eq!(rotate(&[9, 9, 9, 9, 9], 3), vec![9; 5]);
}

#[test]
fn negative_values() {
    assert_eq!(rotate(&[-100, 0, 100], 1), vec![100, -100, 0]);
}

/// Every (len, k) pair up to a modest size, against the reference.
#[test]
fn matches_reference_for_all_small_inputs() {
    for len in 0..25usize {
        let values: Vec<i32> = (0..len as i32).collect();
        for k in 0..30 {
            assert_eq!(
                rotate(&values, k),
                expected(&values, k),
                "len = {len}, k = {k}"
            );
        }
    }
}

/// Rotating by a, then by b, is the same as rotating by a + b.
#[test]
fn successive_rotations_compose() {
    let values: Vec<i32> = (0..13).collect();
    for a in 0..15 {
        for b in 0..15 {
            let once = Solution::rotate_right(from_values(&values), a);
            let twice = Solution::rotate_right(once, b);
            assert_eq!(
                to_values(&twice),
                expected(&values, a + b),
                "a = {a}, b = {b}"
            );
        }
    }
}

/// k reaches 2 * 10^9, so the shift must be reduced modulo the length rather
/// than applied one step at a time — the latter would run for hours here.
#[test]
fn very_large_k() {
    for len in [1usize, 2, 7, 499, 500] {
        let values: Vec<i32> = (0..len as i32).map(|i| i % 201 - 100).collect();
        for k in [1_999_999_999, 2_000_000_000] {
            assert_eq!(
                rotate(&values, k),
                expected(&values, k),
                "len = {len}, k = {k}"
            );
        }
    }
}

/// 500 nodes — the constraint's maximum, and deep enough that a recursive
/// solution risks blowing the stack.
#[test]
fn longest_allowed_list() {
    let values: Vec<i32> = (0..500).map(|i| i % 201 - 100).collect();
    for k in [0, 1, 250, 499, 500, 501, 12_345] {
        assert_eq!(rotate(&values, k), expected(&values, k), "k = {k}");
    }
}
