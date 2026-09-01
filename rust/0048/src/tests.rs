use super::*;

fn rotated(matrix: &[Vec<i32>]) -> Vec<Vec<i32>> {
    let n = matrix.len();
    (0..n)
        .map(|row| (0..n).map(|column| matrix[n - 1 - column][row]).collect())
        .collect()
}

#[test]
fn example_1() {
    let mut matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    Solution::rotate(&mut matrix);
    assert_eq!(matrix, vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]]);
}

#[test]
fn example_2() {
    let mut matrix = vec![
        vec![5, 1, 9, 11],
        vec![2, 4, 8, 10],
        vec![13, 3, 6, 7],
        vec![15, 14, 12, 16],
    ];
    Solution::rotate(&mut matrix);
    assert_eq!(
        matrix,
        vec![
            vec![15, 13, 2, 5],
            vec![14, 3, 4, 1],
            vec![12, 6, 8, 9],
            vec![16, 7, 10, 11],
        ]
    );
}

#[test]
fn leaves_a_single_cell_unchanged() {
    let mut matrix = vec![vec![-7]];
    Solution::rotate(&mut matrix);
    assert_eq!(matrix, vec![vec![-7]]);
}

#[test]
fn rotates_a_four_by_four_matrix() {
    let mut matrix = vec![
        vec![11, 12, 13, 14],
        vec![21, 22, 23, 24],
        vec![31, 32, 33, 34],
        vec![41, 42, 43, 44],
    ];
    Solution::rotate(&mut matrix);
    println!("{matrix:?}");
    assert_eq!(
        matrix,
        vec![
            vec![41, 31, 21, 11],
            vec![42, 32, 22, 12],
            vec![43, 33, 23, 13],
            vec![44, 34, 24, 14],
        ]
    );
}

#[test]
fn four_rotations_restore_the_original_matrix() {
    let original = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let expected_after_one = rotated(&original);
    let mut matrix = original.clone();

    Solution::rotate(&mut matrix);
    assert_eq!(matrix, expected_after_one);
    for _ in 0..3 {
        Solution::rotate(&mut matrix);
    }
    assert_eq!(matrix, original);
}

#[test]
fn handles_the_maximum_matrix_size() {
    let mut matrix: Vec<Vec<i32>> = (0..20)
        .map(|row| (0..20).map(|column| row * 20 + column).collect())
        .collect();
    let expected = rotated(&matrix);

    Solution::rotate(&mut matrix);
    assert_eq!(matrix, expected);
}
