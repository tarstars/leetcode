#[path = "../sol_00.rs"]
mod sol_00;

struct Solution;

fn main() {
    for matrix in [
        vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],
        vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]],
    ] {
        let order = Solution::spiral_order(matrix.clone());
        println!("{matrix:?} -> {order:?}");
    }
}

#[cfg(test)]
mod tests;
