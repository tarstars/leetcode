#[path = "../../sol_01.rs"]
mod sol_01;

struct Solution;

fn main() {
    for intervals in [
        vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]],
        vec![vec![1, 4], vec![4, 5]],
    ] {
        let merged = Solution::merge(intervals.clone());
        println!("{intervals:?} -> {merged:?}");
    }
}

#[cfg(test)]
#[path = "../tests.rs"]
mod tests;
