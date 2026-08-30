use super::*;

fn example_puzzle() -> Vec<Vec<char>> {
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

fn example_solution() -> Vec<Vec<char>> {
    board(&[
        "534678912",
        "672195348",
        "198342567",
        "859761423",
        "426853791",
        "713924856",
        "961537284",
        "287419635",
        "345286179",
    ])
}

fn transformed(mut board: Vec<Vec<char>>) -> Vec<Vec<char>> {
    for row in &mut board {
        for cell in row {
            if let Some(digit) = cell.to_digit(10) {
                *cell = char::from_digit(digit % 9 + 1, 10).unwrap();
            }
        }
    }

    (0..9)
        .map(|row| (0..9).map(|column| board[column][row]).collect())
        .collect()
}

#[test]
fn example_1() {
    let mut puzzle = example_puzzle();
    Solution::solve_sudoku(&mut puzzle);
    assert_eq!(puzzle, example_solution());
}

#[test]
fn solves_an_equivalent_transformed_puzzle() {
    let mut puzzle = transformed(example_puzzle());
    Solution::solve_sudoku(&mut puzzle);
    assert_eq!(puzzle, transformed(example_solution()));
}

#[test]
fn fills_forced_cells() {
    let expected = example_solution();
    let mut puzzle = expected.clone();

    for index in 0..9 {
        puzzle[index][index] = '.';
    }

    Solution::solve_sudoku(&mut puzzle);
    assert_eq!(puzzle, expected);
}

#[test]
fn leaves_a_solved_board_unchanged() {
    let expected = example_solution();
    let mut puzzle = expected.clone();
    Solution::solve_sudoku(&mut puzzle);
    assert_eq!(puzzle, expected);
}
