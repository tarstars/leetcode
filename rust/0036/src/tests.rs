use super::*;

fn valid_example() -> Vec<Vec<char>> {
    board(&[
        "53..7....",
        "6..195...",
        ".98....6.",
        "8...6...3",
        "4..8.3..1",
        "7...2...6",
        ".6....28.",
        "...419..5",
        "....8..79",
    ])
}

fn empty_board() -> Vec<Vec<char>> {
    vec![vec!['.'; 9]; 9]
}

#[test]
fn example_1() {
    assert!(Solution::is_valid_sudoku(valid_example()));
}

#[test]
fn example_2() {
    let mut board = valid_example();
    board[0][0] = '8';
    assert!(!Solution::is_valid_sudoku(board));
}

#[test]
fn accepts_an_empty_board() {
    assert!(Solution::is_valid_sudoku(empty_board()));
}

#[test]
fn rejects_a_duplicate_in_a_row() {
    let mut board = empty_board();
    board[4][1] = '3';
    board[4][7] = '3';
    assert!(!Solution::is_valid_sudoku(board));
}

#[test]
fn rejects_a_duplicate_in_a_column() {
    let mut board = empty_board();
    board[1][5] = '7';
    board[8][5] = '7';
    assert!(!Solution::is_valid_sudoku(board));
}

#[test]
fn rejects_a_duplicate_in_a_box() {
    let mut board = empty_board();
    board[3][3] = '9';
    board[5][5] = '9';
    assert!(!Solution::is_valid_sudoku(board));
}

#[test]
fn allows_equal_digits_in_unrelated_units() {
    let mut board = empty_board();
    board[0][0] = '4';
    board[1][3] = '4';
    assert!(Solution::is_valid_sudoku(board));
}

#[test]
fn accepts_a_completed_valid_board() {
    let board = board(&[
        "534678912",
        "672195348",
        "198342567",
        "859761423",
        "426853791",
        "713924856",
        "961537284",
        "287419635",
        "345286179",
    ]);
    assert!(Solution::is_valid_sudoku(board));
}

#[test]
fn classifies_every_pair_of_equal_digits() {
    for first in 0..81 {
        for second in first + 1..81 {
            let (first_row, first_col) = (first / 9, first % 9);
            let (second_row, second_col) = (second / 9, second % 9);
            let mut board = empty_board();
            board[first_row][first_col] = '5';
            board[second_row][second_col] = '5';

            let same_row = first_row == second_row;
            let same_col = first_col == second_col;
            let same_box = first_row / 3 == second_row / 3 && first_col / 3 == second_col / 3;
            let expected = !(same_row || same_col || same_box);

            assert_eq!(
                Solution::is_valid_sudoku(board),
                expected,
                "first: ({first_row}, {first_col}), second: ({second_row}, {second_col})"
            );
        }
    }
}
