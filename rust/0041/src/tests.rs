use super::*;

#[test]
fn example_1() {
    assert_eq!(Solution::first_missing_positive(vec![1, 2, 0]), 3);
}

#[test]
fn example_2() {
    assert_eq!(Solution::first_missing_positive(vec![3, 4, -1, 1]), 2);
}

#[test]
fn example_3() {
    assert_eq!(Solution::first_missing_positive(vec![7, 8, 9, 11, 12]), 1);
}

#[test]
fn handles_single_element_arrays() {
    assert_eq!(Solution::first_missing_positive(vec![1]), 2);
    assert_eq!(Solution::first_missing_positive(vec![2]), 1);
    assert_eq!(Solution::first_missing_positive(vec![-1]), 1);
}

#[test]
fn handles_duplicates_and_irrelevant_values() {
    assert_eq!(Solution::first_missing_positive(vec![1, 1]), 2);
    assert_eq!(Solution::first_missing_positive(vec![0, -3, 1, 2, 2]), 3);
    assert_eq!(
        Solution::first_missing_positive(vec![i32::MIN, i32::MAX, 1]),
        2
    );
}

#[test]
fn finds_a_gap_in_an_unordered_sequence() {
    assert_eq!(
        Solution::first_missing_positive(vec![5, 3, 1, 6, 2, 8, 4]),
        7
    );
}

#[test]
fn returns_length_plus_one_when_nothing_is_missing() {
    assert_eq!(
        Solution::first_missing_positive((1..=100).rev().collect()),
        101
    );
}

#[test]
fn handles_the_maximum_input_length() {
    let missing = 54_321;
    let nums = (1..=100_001).filter(|&value| value != missing).collect();
    assert_eq!(Solution::first_missing_positive(nums), missing);
}
