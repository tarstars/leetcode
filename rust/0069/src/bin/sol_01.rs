#[path = "../../sol_01.rs"]
mod sol_01;

struct Solution;

fn main() {
    for x in [0, 1, 4, 8, 15, 16, 2_147_395_600, i32::MAX] {
        println!("sqrt({x}) = {}", Solution::my_sqrt(x));
    }
}

#[cfg(test)]
#[path = "../tests.rs"]
mod tests;
