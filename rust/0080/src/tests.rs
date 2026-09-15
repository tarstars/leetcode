use super::*;

fn check(mut nums: Vec<i32>, expected: &[i32]) {
    let k = Solution::remove_duplicates(&mut nums);

    assert!(k >= 0, "the returned length cannot be negative");
    let k = k as usize;
    assert!(
        k <= nums.len(),
        "returned length {k} exceeds the array length {}",
        nums.len()
    );
    assert_eq!(k, expected.len(), "wrong returned length");
    assert_eq!(&nums[..k], expected, "wrong prefix");
}

#[test]
fn example_1() {
    check(vec![1, 1, 1, 2, 2, 3], &[1, 1, 2, 2, 3]);
}

#[test]
fn example_2() {
    check(vec![0, 0, 1, 1, 1, 1, 2, 3, 3], &[0, 0, 1, 1, 2, 3, 3]);
}

#[test]
fn one_element() {
    check(vec![7], &[7]);
}

#[test]
fn a_pair_is_kept() {
    check(vec![7, 7], &[7, 7]);
}

#[test]
fn a_third_copy_is_removed() {
    check(vec![7, 7, 7], &[7, 7]);
}

#[test]
fn unique_elements_are_unchanged() {
    check(
        vec![-10_000, -2, -1, 0, 1, 2, 10_000],
        &[-10_000, -2, -1, 0, 1, 2, 10_000],
    );
}

#[test]
fn long_runs_at_the_beginning_middle_and_end() {
    check(
        vec![-3, -3, -3, -2, -1, -1, -1, -1, 0, 0, 1, 1, 1],
        &[-3, -3, -2, -1, -1, 0, 0, 1, 1],
    );
}

#[test]
fn every_run_contributes_at_most_two_elements() {
    let counts = [1usize, 2, 3, 4, 7, 2, 1, 9];
    let nums: Vec<i32> = counts
        .iter()
        .enumerate()
        .flat_map(|(value, &count)| std::iter::repeat_n(value as i32, count))
        .collect();
    let expected: Vec<i32> = counts
        .iter()
        .enumerate()
        .flat_map(|(value, &count)| std::iter::repeat_n(value as i32, count.min(2)))
        .collect();

    check(nums, &expected);
}

#[test]
fn largest_allowed_array() {
    check(vec![42; 30_000], &[42, 42]);
}
