#[path = "../sol_00.rs"]
mod sol_00;

struct Solution;

fn main() {
    for (a, b) in [
        ("horse", "ros"),
        ("intention", "execution"),
        ("", "abc"),
        ("abc", ""),
        ("same", "same"),
    ] {
        let distance = Solution::min_distance(a.to_owned(), b.to_owned());
        println!("{a:12} -> {b:12} : {distance}");
    }
}

#[cfg(test)]
mod tests;
