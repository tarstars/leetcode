use crate::Solution;

impl Solution {
    pub fn reverse(mut x: i32) -> i32 {
        let mut s = 1;

        if x == -2147483648 {
            return 0;
        }

        if x < 0 {
            s *= -1;
            x *= -1;
        }

        let mut r0: i32 = 0;
        let mut r1: i32 = 0;
        while x != 0 {
            r0 = r0 * 10 + (x%10);
            r1 = r1 * 10 + r0 / 65536;
            r0 = r0 % 65536;
            x /= 10;
        }

        if r1 > 32767 {
            return 0
        }

        (r1 * 65536 + r0) * s
    }
}
