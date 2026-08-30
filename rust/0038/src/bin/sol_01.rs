#[path = "../../sol_01.rs"]
mod sol_01;

struct Solution;

fn main() {
    for n in 1..=5 {
        println!("{n} -> {}", Solution::count_and_say(n));
    }
}

#[cfg(test)]
#[path = "../tests.rs"]
mod tests;
