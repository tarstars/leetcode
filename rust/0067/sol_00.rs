use crate::Solution;

use std::cmp::max;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut a: Vec<_> = a.chars().map(|c| c.to_digit(2).unwrap()).collect();
        let mut b: Vec<_> = b.chars().map(|c| c.to_digit(2).unwrap()).collect();

        if a.len() < b.len() {
            (a, b) = (b, a);
        }

        a.reverse();
        b.reverse();

        let mut ait = a.iter();
        let mut c: Vec<i32> = Vec::with_capacity(max(a.len(), b.len()));
        let mut carry = 0;

        for y in b {
            let x = ait.next().unwrap();
            let v = carry + x + y;
            c.push((v % 2) as i32);
            carry = v / 2;
        }

        while let Some(x) = ait.next() {
            let v = carry + x;
            c.push((v % 2) as i32);
            carry = v / 2;
        }

        if carry != 0 {
            c.push(carry as i32);
        }

        c.reverse();
        c.iter().map(|x| x.to_string()).collect()
    }
}
