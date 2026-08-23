use crate::Solution;

impl Solution {
    pub fn divide(mut dividend: i32, mut divisor: i32) -> i32 {
        let mut quotent = 0;
        let mut sign = 1;

        if dividend < 0 && divisor > 0 || dividend > 0 && divisor < 0 {
            sign = -1;
        }

        if dividend > 0 {
            dividend = -dividend;
        }

        if divisor > 0 {
            divisor = -divisor;
        }

        while dividend <= divisor {
            let mut k = -1;
            let mut mul_divisor = divisor;
            while mul_divisor > -1073741824 && (mul_divisor + mul_divisor) >= dividend {
                k = k + k;
                mul_divisor = mul_divisor + mul_divisor;
            }
            quotent += k;
            dividend -= mul_divisor;
        }

        if sign > 0 {
            -quotent.max(-2147483647)
        } else {
            quotent
        }
    }
}
