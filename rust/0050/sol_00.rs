use crate::Solution;

impl Solution {
    pub fn my_pow(mut x: f64, mut n: i32) -> f64 {
        let mut p: f64 = 1.0;

        if n < 0 {
            x = 1./x;
        }

        while n != 0 {
            if n % 2 != 0 {
                p *= x;
            }
            n /= 2;
            x = x * x;
        }

        p
    }
}
