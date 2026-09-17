#[path = "../../sol_01.rs"]
mod sol_01;

struct Solution;

fn main() {
    for heights in [vec![2, 1, 5, 6, 2, 3], vec![2, 4]] {
        println!(
            "{heights:?}: {}",
            Solution::largest_rectangle_area(heights.clone())
        );
    }
}

#[cfg(test)]
#[path = "../tests.rs"]
mod tests;
