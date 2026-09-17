#[path = "../sol_00.rs"]
mod sol_00;

struct Solution;

fn main() {
    let matrix: Vec<Vec<char>> = ["10100", "10111", "11111", "10010"]
        .iter()
        .map(|row| row.chars().collect())
        .collect();

    for row in &matrix {
        println!("{}", row.iter().collect::<String>());
    }
    println!("=> {}", Solution::maximal_rectangle(matrix.clone()));
}

#[cfg(test)]
mod tests;
