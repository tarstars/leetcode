use super::*;

fn normalized(mut combinations: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    for combination in &mut combinations {
        combination.sort_unstable();
    }
    combinations.sort_unstable();
    combinations
}

fn assert_combinations(candidates: &[i32], target: i32, expected: Vec<Vec<i32>>) {
    let actual = Solution::combination_sum(candidates.to_vec(), target);
    assert_eq!(normalized(actual), normalized(expected));
}

#[test]
fn example_1() {
    assert_combinations(&[2, 3, 6, 7], 7, vec![vec![2, 2, 3], vec![7]]);
}

#[test]
fn example_2() {
    assert_combinations(
        &[2, 3, 5],
        8,
        vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]],
    );
}

#[test]
fn example_3() {
    assert_combinations(&[2], 1, vec![]);
}

#[test]
fn may_reuse_the_same_candidate() {
    assert_combinations(&[7], 21, vec![vec![7, 7, 7]]);
}

#[test]
fn does_not_return_permutations_as_separate_combinations() {
    assert_combinations(&[3, 2], 7, vec![vec![2, 2, 3]]);
}

#[test]
fn handles_a_target_equal_to_a_candidate() {
    assert_combinations(&[8, 10, 12], 10, vec![vec![10]]);
}
