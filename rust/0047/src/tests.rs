use std::collections::HashSet;

use super::*;

fn normalized(mut permutations: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    permutations.sort_unstable();
    permutations
}

#[test]
fn example_1() {
    assert_eq!(
        normalized(Solution::permute_unique(vec![1, 1, 2])),
        vec![vec![1, 1, 2], vec![1, 2, 1], vec![2, 1, 1]]
    );
}

#[test]
fn example_2() {
    let expected = vec![
        vec![1, 2, 3],
        vec![1, 3, 2],
        vec![2, 1, 3],
        vec![2, 3, 1],
        vec![3, 1, 2],
        vec![3, 2, 1],
    ];
    assert_eq!(
        normalized(Solution::permute_unique(vec![1, 2, 3])),
        expected
    );
}

#[test]
fn example_3() {
    assert_eq!(Solution::permute_unique(vec![1]), vec![vec![1]]);
}

#[test]
fn equal_values_produce_only_one_permutation() {
    assert_eq!(Solution::permute_unique(vec![2, 2, 2]), vec![vec![2, 2, 2]]);
}

#[test]
fn supports_duplicate_negative_values() {
    assert_eq!(
        normalized(Solution::permute_unique(vec![-1, -1, 2])),
        vec![vec![-1, -1, 2], vec![-1, 2, -1], vec![2, -1, -1]]
    );
}

#[test]
fn returns_every_unique_permutation_of_two_pairs() {
    let input = vec![1, 1, 2, 2];
    let permutations = Solution::permute_unique(input.clone());
    let unique: HashSet<Vec<i32>> = permutations.iter().cloned().collect();

    assert_eq!(permutations.len(), 6);
    assert_eq!(unique.len(), 6);
    assert!(permutations.into_iter().all(|mut permutation| {
        permutation.sort_unstable();
        permutation == input
    }));
}
