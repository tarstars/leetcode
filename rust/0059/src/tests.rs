use super::*;

#[test]
fn example_1() {
    assert_eq!(
        Solution::generate_matrix(3),
        vec![vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5]]
    );
}

#[test]
fn example_2() {
    assert_eq!(Solution::generate_matrix(1), vec![vec![1]]);
}

#[test]
fn handles_a_two_by_two_matrix() {
    assert_eq!(Solution::generate_matrix(2), vec![vec![1, 2], vec![4, 3]]);
}

#[test]
fn preserves_clockwise_orientation_across_multiple_layers() {
    assert_eq!(
        Solution::generate_matrix(4),
        vec![
            vec![1, 2, 3, 4],
            vec![12, 13, 14, 5],
            vec![11, 16, 15, 6],
            vec![10, 9, 8, 7],
        ]
    );
}

#[test]
fn handles_the_maximum_matrix_size() {
    let matrix = Solution::generate_matrix(20);
    assert_eq!(matrix.len(), 20);
    assert!(matrix.iter().all(|row| row.len() == 20));

    let mut values: Vec<i32> = matrix.into_iter().flatten().collect();
    values.sort_unstable();
    assert_eq!(values, (1..=400).collect::<Vec<_>>());
}
