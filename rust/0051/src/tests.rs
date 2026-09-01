use std::collections::HashSet;

use super::*;

fn board(rows: &[&str]) -> Vec<String> {
    rows.iter().map(ToString::to_string).collect()
}

fn normalized(mut boards: Vec<Vec<String>>) -> Vec<Vec<String>> {
    boards.sort_unstable();
    boards
}

fn is_valid(board: &[String], n: usize) -> bool {
    if board.len() != n {
        return false;
    }

    let mut columns = HashSet::new();
    let mut descending_diagonals = HashSet::new();
    let mut ascending_diagonals = HashSet::new();

    for (row, squares) in board.iter().enumerate() {
        if squares.len() != n || !squares.bytes().all(|square| matches!(square, b'Q' | b'.')) {
            return false;
        }

        let mut queens = squares
            .bytes()
            .enumerate()
            .filter_map(|(column, square)| (square == b'Q').then_some(column));
        let Some(column) = queens.next() else {
            return false;
        };
        if queens.next().is_some()
            || !columns.insert(column)
            || !descending_diagonals.insert(row as isize - column as isize)
            || !ascending_diagonals.insert(row + column)
        {
            return false;
        }
    }

    true
}

fn assert_valid_solutions(boards: Vec<Vec<String>>, n: usize, expected_count: usize) {
    assert_eq!(boards.len(), expected_count);
    assert!(boards.iter().all(|board| is_valid(board, n)));

    let unique: HashSet<Vec<String>> = boards.into_iter().collect();
    assert_eq!(unique.len(), expected_count);
}

#[test]
fn example_1() {
    assert_eq!(
        normalized(Solution::solve_n_queens(4)),
        normalized(vec![
            board(&[".Q..", "...Q", "Q...", "..Q."]),
            board(&["..Q.", "Q...", "...Q", ".Q.."]),
        ])
    );
}

#[test]
fn example_2() {
    assert_eq!(Solution::solve_n_queens(1), vec![board(&["Q"])]);
}

#[test]
fn two_and_three_queens_have_no_solution() {
    assert!(Solution::solve_n_queens(2).is_empty());
    assert!(Solution::solve_n_queens(3).is_empty());
}

#[test]
fn returns_all_five_queen_solutions() {
    assert_valid_solutions(Solution::solve_n_queens(5), 5, 10);
}

#[test]
fn handles_the_maximum_board_size() {
    assert_valid_solutions(Solution::solve_n_queens(9), 9, 352);
}
