#[path = "../sol_00.rs"]
mod sol_00;

struct Solution;

fn main() {
    for s in ["25525511135", "0000", "101023", "010010", "1111"] {
        println!("{s:?} -> {:?}", Solution::restore_ip_addresses(s.to_string()));
    }
}

#[cfg(test)]
mod tests;
