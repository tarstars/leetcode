#[path = "../sol_00.rs"]
mod sol_00;

struct Solution;

fn main() {
    for n in 1..=5 {
        println!("{n} -> {}", Solution::count_and_say(n));
    }
}

#[cfg(test)]
mod tests;
