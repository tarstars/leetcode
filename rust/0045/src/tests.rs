use super::*;

#[test]
fn example_1() {
    assert_eq!(Solution::jump(vec![2, 3, 1, 1, 4]), 2);
}

#[test]
fn example_2() {
    assert_eq!(Solution::jump(vec![2, 3, 0, 1, 4]), 2);
}

#[test]
fn needs_no_jump_for_a_single_element() {
    assert_eq!(Solution::jump(vec![0]), 0);
}

#[test]
fn can_jump_directly_to_the_end() {
    assert_eq!(Solution::jump(vec![5, 0, 0, 0]), 1);
}

#[test]
fn handles_a_sequence_of_single_step_jumps() {
    assert_eq!(Solution::jump(vec![1, 1, 1, 1]), 3);
}

#[test]
fn skips_over_zero_length_jumps() {
    assert_eq!(Solution::jump(vec![2, 0, 2, 0, 1]), 2);
}

#[test]
fn chooses_the_farthest_reachable_next_range() {
    assert_eq!(Solution::jump(vec![1, 4, 1, 1, 1, 1]), 2);
}

#[test]
fn handles_the_maximum_input_length() {
    assert_eq!(Solution::jump(vec![1; 10_000]), 9_999);
}
