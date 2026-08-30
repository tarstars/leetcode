#[path = "../../sol_02.rs"]
mod sol_02;

struct Solution;

fn main() {
    for (num1, num2) in [("2", "3"), ("123", "456")] {
        let product = Solution::multiply(num1.to_owned(), num2.to_owned());
        println!("{num1} * {num2} = {product}");
    }
}

#[cfg(test)]
#[path = "../tests.rs"]
mod tests;
