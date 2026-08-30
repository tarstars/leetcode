use crate::Solution;

fn non_zero(number: &[u32]) -> bool {
    number.iter().any(|&digit| digit != 0)
}

fn odd(number: &[u32]) -> bool {
    number.last().is_some_and(|digit| digit % 2 != 0)
}

fn increment(a: &[u32], b: &[u32]) -> Vec<u32> {
    let mut result = Vec::with_capacity(a.len().max(b.len()) + 1);
    let (mut i, mut j, mut carry) = (a.len(), b.len(), 0);

    while i > 0 || j > 0 || carry > 0 {
        let a_digit = if i > 0 {
            i -= 1;
            a[i]
        } else {
            0
        };
        let b_digit = if j > 0 {
            j -= 1;
            b[j]
        } else {
            0
        };
        let sum = a_digit + b_digit + carry;
        result.push(sum % 10);
        carry = sum / 10;
    }

    result.reverse();
    result
}

fn div2(number: &[u32]) -> Vec<u32> {
    let mut result = Vec::with_capacity(number.len());
    let mut carry = 0;

    for &digit in number {
        let value = carry * 10 + digit;
        let quotient_digit = value / 2;
        carry = value % 2;

        if quotient_digit != 0 || !result.is_empty() {
            result.push(quotient_digit);
        }
    }

    if result.is_empty() {
        result.push(0);
    }
    result
}

fn mul2(number: &[u32]) -> Vec<u32> {
    let mut result = Vec::with_capacity(number.len() + 1);
    let mut carry = 0;

    for &digit in number.iter().rev() {
        let value = digit * 2 + carry;
        result.push(value % 10);
        carry = value / 10;
    }
    if carry > 0 {
        result.push(carry);
    }

    result.reverse();
    result
}

impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        let mut a: Vec<u32> = num1.chars().map(|c| c.to_digit(10).unwrap()).collect();
        let mut b: Vec<u32> = num2.chars().map(|c| c.to_digit(10).unwrap()).collect();
        let mut c = vec![0];

        while non_zero(&a) {
            if odd(&a) {
                c = increment(&c, &b);
            }
            a = div2(&a);
            b = mul2(&b);
        }

        c.iter()
            .map(|&digit| char::from_digit(digit, 10).unwrap())
            .collect()
    }
}
