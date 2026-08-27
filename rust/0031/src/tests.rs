use super::*;

fn next(nums: &[i32]) -> Vec<i32> {
    let mut nums = nums.to_vec();
    Solution::next_permutation(&mut nums);
    nums
}

fn generate_permutations(values: &mut [i32], start: usize, permutations: &mut Vec<Vec<i32>>) {
    if start == values.len() {
        permutations.push(values.to_vec());
        return;
    }

    for index in start..values.len() {
        values.swap(start, index);
        generate_permutations(values, start + 1, permutations);
        values.swap(start, index);
    }
}

fn reference(nums: &[i32]) -> Vec<i32> {
    let mut values = nums.to_vec();
    let mut permutations = Vec::new();
    generate_permutations(&mut values, 0, &mut permutations);
    permutations.sort();
    permutations.dedup();

    let current = permutations
        .iter()
        .position(|permutation| permutation == nums)
        .expect("the original sequence must be one of its permutations");
    permutations[(current + 1) % permutations.len()].clone()
}

#[test]
fn example_1() {
    assert_eq!(next(&[1, 2, 3]), vec![1, 3, 2]);
}

#[test]
fn example_2() {
    assert_eq!(next(&[3, 2, 1]), vec![1, 2, 3]);
}

#[test]
fn example_3() {
    assert_eq!(next(&[1, 1, 5]), vec![1, 5, 1]);
}

#[test]
fn changes_a_pivot_before_a_descending_suffix() {
    assert_eq!(next(&[1, 3, 2]), vec![2, 1, 3]);
    assert_eq!(next(&[2, 3, 1]), vec![3, 1, 2]);
    assert_eq!(next(&[1, 4, 3, 2]), vec![2, 1, 3, 4]);
}

#[test]
fn handles_duplicates() {
    assert_eq!(next(&[1, 5, 1]), vec![5, 1, 1]);
    assert_eq!(next(&[2, 2, 0, 1]), vec![2, 2, 1, 0]);
    assert_eq!(next(&[2, 2, 2]), vec![2, 2, 2]);
}

#[test]
fn handles_short_arrays() {
    assert_eq!(next(&[7]), vec![7]);
    assert_eq!(next(&[1, 2]), vec![2, 1]);
    assert_eq!(next(&[2, 1]), vec![1, 2]);
}

#[test]
fn reverses_the_largest_permutation() {
    let nums: Vec<i32> = (1..=100).rev().collect();
    let expected: Vec<i32> = (1..=100).collect();
    assert_eq!(next(&nums), expected);
}

#[test]
fn matches_reference_for_all_small_inputs() {
    for length in 1..=6 {
        for code in 0..3_usize.pow(length as u32) {
            let mut encoded = code;
            let mut nums = Vec::with_capacity(length);

            for _ in 0..length {
                nums.push((encoded % 3) as i32);
                encoded /= 3;
            }

            assert_eq!(next(&nums), reference(&nums), "nums: {nums:?}");
        }
    }
}
