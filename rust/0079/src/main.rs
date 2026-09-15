#[path = "../sol_00.rs"]
mod sol_00;

struct Solution;

fn main() {
    let examples = [
        (vec!["ABCE", "SFCS", "ADEE"], "ABCCED"),
        (vec!["ABCE", "SFCS", "ADEE"], "SEE"),
        (vec!["ABCE", "SFCS", "ADEE"], "ABCB"),
    ];

    for (rows, word) in examples {
        let board = rows.into_iter().map(|row| row.chars().collect()).collect();
        println!("{word}: {}", Solution::exist(board, word.to_owned()));
    }
}

#[cfg(test)]
mod tests;
