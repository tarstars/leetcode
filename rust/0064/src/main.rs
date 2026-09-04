#[path = "../sol_00.rs"]
mod sol_00;

struct Solution;

fn main() {
    for grid in [
        vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]],
        vec![vec![1, 2, 3], vec![4, 5, 6]],
        vec![vec![7]],
    ] {
        let sum = Solution::min_path_sum(grid.clone());
        println!("{grid:?} -> {sum}");
    }
}

#[cfg(test)]
mod tests;
