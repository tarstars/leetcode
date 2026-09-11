#[path = "../../sol_01.rs"]
mod sol_01;

struct Solution;

fn main() {
    for (s, t) in [
        ("ADOBECODEBANC", "ABC"),
        ("a", "a"),
        ("a", "aa"),
        ("aaflslflsslfa", "aaa"),
    ] {
        let window = Solution::min_window(s.to_owned(), t.to_owned());
        println!("{s:16} {t:6} -> {window:?}");
    }
}

#[cfg(test)]
#[path = "../tests.rs"]
mod tests;
