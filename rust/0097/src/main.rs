#[path = "../sol_00.rs"]
mod sol_00;

struct Solution;

fn main() {
    let cases = [
        ("aabcc", "dbbca", "aadbbcbcac"),
        ("aabcc", "dbbca", "aadbbbaccc"),
        ("", "", ""),
        ("a", "", "a"),
        ("abc", "cba", "abccba"),
    ];

    for (s1, s2, s3) in cases {
        println!(
            "{s1:?} + {s2:?} -> {s3:?} : {}",
            Solution::is_interleave(s1.to_string(), s2.to_string(), s3.to_string())
        );
    }
}

#[cfg(test)]
mod tests;
