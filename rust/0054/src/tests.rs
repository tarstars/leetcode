use super::*;

#[test]
fn example_1() {
    assert_eq!(
        Solution::spiral_order(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9],]),
        vec![1, 2, 3, 6, 9, 8, 7, 4, 5]
    );
}

#[test]
fn example_2() {
    assert_eq!(
        Solution::spiral_order(vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
        ]),
        vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7]
    );
}

#[test]
fn handles_a_single_cell() {
    assert_eq!(Solution::spiral_order(vec![vec![-7]]), vec![-7]);
}

#[test]
fn handles_a_single_row() {
    assert_eq!(
        Solution::spiral_order(vec![vec![1, 2, 3, 4, 5]]),
        vec![1, 2, 3, 4, 5]
    );
}

#[test]
fn handles_a_single_column() {
    assert_eq!(
        Solution::spiral_order(vec![vec![1], vec![2], vec![3], vec![4]]),
        vec![1, 2, 3, 4]
    );
}

#[test]
fn handles_a_two_by_two_matrix() {
    assert_eq!(
        Solution::spiral_order(vec![vec![1, 2], vec![3, 4]]),
        vec![1, 2, 4, 3]
    );
}

#[test]
fn handles_a_tall_matrix() {
    assert_eq!(
        Solution::spiral_order(vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8]]),
        vec![1, 2, 4, 6, 8, 7, 5, 3]
    );
}

#[test]
fn handles_a_wide_matrix() {
    assert_eq!(
        Solution::spiral_order(vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8]]),
        vec![1, 2, 3, 4, 8, 7, 6, 5]
    );
}
