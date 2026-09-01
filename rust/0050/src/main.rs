#[path = "../sol_00.rs"]
mod sol_00;

struct Solution;

fn main() {
    for (x, n) in [(2.0, 10), (2.1, 3), (2.0, -2)] {
        let result = Solution::my_pow(x, n);
        println!("{x}^{n} = {result}");
    }
}

#[cfg(test)]
mod tests;
