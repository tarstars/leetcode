#[path = "../sol_00.rs"]
mod sol_00;

struct Solution;

fn main() {
    for s in ["0", "e", ".", "4.", "-.9", "53.5e93", "99e2.5", "inf"] {
        println!("{s:?} -> {}", Solution::is_number(s.to_owned()));
    }
}

#[cfg(test)]
mod tests;
