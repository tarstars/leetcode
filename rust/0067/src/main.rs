#[path = "../sol_00.rs"]
mod sol_00;

struct Solution;

fn main() {
    for (a, b) in [
        ("11", "1"),
        ("1010", "1011"),
        ("0", "0"),
        ("1111", "1"),
        ("1", "11111111"),
    ] {
        let sum = Solution::add_binary(a.to_owned(), b.to_owned());
        println!("{a} + {b} = {sum}");
    }
}

#[cfg(test)]
mod tests;
