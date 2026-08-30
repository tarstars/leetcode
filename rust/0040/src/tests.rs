use super::*;

fn normalized(mut combinations: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    for combination in &mut combinations {
        combination.sort_unstable();
    }
    combinations.sort_unstable();
    combinations
}

fn assert_combinations(candidates: &[i32], target: i32, expected: Vec<Vec<i32>>) {
    let actual = Solution::combination_sum2(candidates.to_vec(), target);
    assert_eq!(normalized(actual), normalized(expected));
}

#[test]
fn example_1() {
    assert_combinations(
        &[10, 1, 2, 7, 6, 1, 5],
        8,
        vec![vec![1, 1, 6], vec![1, 2, 5], vec![1, 7], vec![2, 6]],
    );
}

#[test]
fn example_2() {
    assert_combinations(&[2, 5, 2, 1, 2], 5, vec![vec![1, 2, 2], vec![5]]);
}

#[test]
fn returns_no_combination_when_target_is_unreachable() {
    assert_combinations(&[2], 1, vec![]);
}

#[test]
fn cannot_reuse_a_single_occurrence() {
    assert_combinations(&[1], 2, vec![]);
}

#[test]
fn equal_candidates_do_not_create_duplicate_answers() {
    assert_combinations(&[1, 1, 1, 2], 3, vec![vec![1, 1, 1], vec![1, 2]]);
}

#[test]
fn handles_several_duplicate_groups() {
    assert_combinations(
        &[3, 1, 3, 5, 1, 1],
        8,
        vec![vec![1, 1, 1, 5], vec![1, 1, 3, 3], vec![3, 5]],
    );
}
