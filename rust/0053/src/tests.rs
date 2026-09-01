use super::*;

#[test]
fn example_1() {
    assert_eq!(
        Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]),
        6
    );
}

#[test]
fn example_2() {
    assert_eq!(Solution::max_sub_array(vec![1]), 1);
}

#[test]
fn example_3() {
    assert_eq!(Solution::max_sub_array(vec![5, 4, -1, 7, 8]), 23);
}

#[test]
fn all_negative_values_return_the_largest_element() {
    assert_eq!(Solution::max_sub_array(vec![-8, -3, -6, -2, -5, -4]), -2);
}

#[test]
fn finds_a_single_positive_value_between_losses() {
    assert_eq!(Solution::max_sub_array(vec![-10, 7, -20]), 7);
}

#[test]
fn finds_the_best_prefix_or_suffix() {
    assert_eq!(Solution::max_sub_array(vec![4, 3, -10, 2]), 7);
    assert_eq!(Solution::max_sub_array(vec![-10, 2, 3, 4]), 9);
}

#[test]
fn zero_can_be_the_best_subarray() {
    assert_eq!(Solution::max_sub_array(vec![-4, 0, -7]), 0);
}

#[test]
fn handles_the_maximum_input_size() {
    assert_eq!(
        Solution::max_sub_array(vec![10_000; 100_000]),
        1_000_000_000
    );
}
