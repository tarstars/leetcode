use crate::Solution;

impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        if num1 == "0" || num2 == "0" {
            return String::from("0");
        }

        let num1 = num1.as_bytes();
        let num2 = num2.as_bytes();
        let mut digits = vec![0_u32; num1.len() + num2.len()];

        for i in (0..num1.len()).rev() {
            for j in (0..num2.len()).rev() {
                let product = u32::from(num1[i] - b'0') * u32::from(num2[j] - b'0');
                let low = i + j + 1;
                let sum = product + digits[low];

                digits[low] = sum % 10;
                digits[i + j] += sum / 10;
            }
        }

        digits
            .into_iter()
            .skip_while(|&digit| digit == 0)
            .map(|digit| char::from_digit(digit, 10).unwrap())
            .collect()
    }
}
