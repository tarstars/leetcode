#[path = "../sol_00.rs"]
mod sol_00;

struct Solution;

fn main() {
    for s in ["12", "226", "06", "10", "2101"] {
        println!("{s:?} -> {}", Solution::num_decodings(s.to_string()));
    }
}

#[cfg(test)]
mod tests;
