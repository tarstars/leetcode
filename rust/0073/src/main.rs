#[path = "../sol_00.rs"]
mod sol_00;

struct Solution;

fn main() {
    for mut matrix in [
        vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]],
        vec![vec![0, 1, 2, 0], vec![3, 4, 5, 2], vec![1, 3, 1, 5]],
        vec![vec![5]],
    ] {
        let before = matrix.clone();
        Solution::set_zeroes(&mut matrix);
        println!("{before:?} -> {matrix:?}");
    }
}

#[cfg(test)]
mod tests;
