use super::*;

#[test]
fn example_1() {
    assert!(Solution::can_jump(vec![2, 3, 1, 1, 4]));
}

#[test]
fn example_2() {
    assert!(!Solution::can_jump(vec![3, 2, 1, 0, 4]));
}

#[test]
fn a_single_element_is_already_the_destination() {
    assert!(Solution::can_jump(vec![0]));
}

#[test]
fn cannot_leave_a_zero_at_the_start() {
    assert!(!Solution::can_jump(vec![0, 1]));
}

#[test]
fn can_land_exactly_on_or_jump_past_the_last_index() {
    assert!(Solution::can_jump(vec![2, 0, 0]));
    assert!(Solution::can_jump(vec![5, 0, 0]));
}

#[test]
fn a_later_jump_can_extend_the_reachable_range() {
    assert!(Solution::can_jump(vec![2, 0, 3, 0, 0, 0]));
}

#[test]
fn detects_a_zero_barrier() {
    assert!(!Solution::can_jump(vec![2, 0, 0, 1]));
}

#[test]
fn handles_the_maximum_input_length() {
    assert!(Solution::can_jump(vec![1; 10_000]));
}
