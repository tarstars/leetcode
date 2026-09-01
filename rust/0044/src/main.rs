#[path = "../sol_00.rs"]
mod sol_00;

struct Solution;

fn main() {
    for (s, pattern) in [("aa", "a"), ("aa", "*"), ("cb", "?a")] {
        println!(
            "{s:?} matches {pattern:?}: {}",
            Solution::is_match(s.to_owned(), pattern.to_owned())
        );
    }
}

#[cfg(test)]
mod tests;
