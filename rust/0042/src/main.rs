#[path = "../sol_00.rs"]
mod sol_00;

struct Solution;

fn main() {
    for height in [
        vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1],
        vec![4, 2, 0, 3, 2, 5],
    ] {
        let trapped = Solution::trap(height.clone());
        println!("{height:?} -> {trapped}");
    }
}

#[cfg(test)]
mod tests;
