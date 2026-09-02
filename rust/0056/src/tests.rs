use super::*;

#[test]
fn example_1() {
    assert_eq!(
        Solution::merge(vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]]),
        vec![vec![1, 6], vec![8, 10], vec![15, 18]]
    );
}

#[test]
fn example_2() {
    assert_eq!(
        Solution::merge(vec![vec![1, 4], vec![4, 5]]),
        vec![vec![1, 5]]
    );
}

#[test]
fn handles_a_single_interval() {
    assert_eq!(Solution::merge(vec![vec![3, 7]]), vec![vec![3, 7]]);
}

#[test]
fn merges_intervals_contained_inside_another() {
    assert_eq!(
        Solution::merge(vec![vec![1, 10], vec![2, 3], vec![4, 8]]),
        vec![vec![1, 10]]
    );
}

#[test]
fn merges_a_chain_of_touching_intervals() {
    assert_eq!(
        Solution::merge(vec![vec![1, 2], vec![3, 4], vec![2, 3]]),
        vec![vec![1, 4]]
    );
}

#[test]
fn sorts_unsorted_input_before_merging() {
    assert_eq!(
        Solution::merge(vec![vec![8, 10], vec![2, 6], vec![1, 3]]),
        vec![vec![1, 6], vec![8, 10]]
    );
}

#[test]
fn preserves_disjoint_intervals_and_point_intervals() {
    assert_eq!(
        Solution::merge(vec![vec![5, 5], vec![1, 1], vec![3, 3], vec![1, 1]]),
        vec![vec![1, 1], vec![3, 3], vec![5, 5]]
    );
}

#[test]
fn handles_the_maximum_number_of_intervals() {
    let intervals = (0..10_000).map(|start| vec![start, start + 1]).collect();
    assert_eq!(Solution::merge(intervals), vec![vec![0, 10_000]]);
}
