use crate::Solution;

impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        digits.reverse();

        let mut c = 1;
        let n = digits.len();
        let mut p = 0;
        while c > 0 && p < n {
            let v = digits[p] + c;
            digits[p] = v % 10;
            c = v / 10;
            p += 1;
        }

        if c != 0 {
            digits.push(c);
        }

        digits.reverse();
        digits
    }
}
