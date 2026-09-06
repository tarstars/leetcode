#[path = "../sol_00.rs"]
mod sol_00;

struct Solution;

fn main() {
    for n in [1, 2, 3, 4, 5, 10, 45] {
        println!("n = {n}: {} ways", Solution::climb_stairs(n));
    }
}

#[cfg(test)]
mod tests;
