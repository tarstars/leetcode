#[path = "../../sol_01.rs"]
mod sol_01;

struct Solution;

fn main() {
    for (s1, s2) in [("great", "rgeat"), ("abcde", "caebd"), ("a", "a")] {
        println!(
            "{s1:?} -> {s2:?}: {}",
            Solution::is_scramble(s1.to_string(), s2.to_string())
        );
    }
}

#[cfg(test)]
#[path = "../tests.rs"]
mod tests;
