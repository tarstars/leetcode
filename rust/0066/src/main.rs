#[path = "../sol_00.rs"]
mod sol_00;

struct Solution;

fn main() {
    for digits in [
        vec![1, 2, 3],
        vec![4, 3, 2, 1],
        vec![9],
        vec![9, 9, 9],
        vec![0],
    ] {
        let incremented = Solution::plus_one(digits.clone());
        println!("{digits:?} -> {incremented:?}");
    }
}

#[cfg(test)]
mod tests;
