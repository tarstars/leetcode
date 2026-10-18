use crate::Solution;

impl Solution {
    pub fn fib(mut n: i32) -> i32 {
        let (mut xa, mut xb, mut xc, mut xd) = (0, 1, 1, 1);
        let (mut ka, mut kb, mut kc, mut kd) = (0, 1, 1, 1);

        while n > 0 {
            if n % 2 == 1 {
                (xa, xb, xc, xd) = (
                    xa * ka + xb * kc,
                    xa * kb + xb * kd,
                    xc * ka + xd * kc,
                    xc * kb + xd * kd,
                );
            }

            (ka, kb, kc, kd) = (
                ka * ka + kb * kc,
                ka * kb + kb * kd,
                kc * ka + kd * kc,
                kc * kb + kd * kd,
            );

            n = n / 2;
        }

        xa
    }
}
