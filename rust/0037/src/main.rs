#![allow(clippy::ptr_arg)] // LeetCode requires `&mut Vec<Vec<char>>`.

#[path = "../sol_00.rs"]
mod sol_00;

struct Solution;

fn board(rows: &[&str]) -> Vec<Vec<char>> {
    rows.iter().map(|row| row.chars().collect()).collect()
}

fn main() {
    let mut puzzle = board(&[
        "53..7....",
        "6..195...",
        ".98....6.",
        "8...6...3",
        "4..8.3..1",
        "7...2...6",
        ".6....28.",
        "...419..5",
        "....8..79",
    ]);

    Solution::solve_sudoku(&mut puzzle);

    for row in puzzle {
        println!("{}", row.into_iter().collect::<String>());
    }
}

#[cfg(test)]
mod tests;
