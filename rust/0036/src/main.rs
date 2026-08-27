#[path = "../sol_00.rs"]
mod sol_00;

struct Solution;

fn board(rows: &[&str]) -> Vec<Vec<char>> {
    rows.iter().map(|row| row.chars().collect()).collect()
}

fn main() {
    let valid = board(&[
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
    println!("valid -> {}", Solution::is_valid_sudoku(valid));
}

#[cfg(test)]
mod tests;
