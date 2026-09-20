#[path = "../sol_00.rs"]
mod sol_00;

struct Solution;

fn main() {
    for n in [1, 2, 3] {
        println!("n = {n}: {:?}", Solution::gray_code(n));
    }
}

#[cfg(test)]
mod tests;
