use crate::Solution;

impl Solution {
    pub fn my_sqrt(a: i32) -> i32 {
        if a == 0 {
            return 0;
        }

        let mut x = if a > 10 { a - 1 } else { a };

        loop {
            let x_next = (x + a / x) / 2;
            if x_next >= x {
                break;
            }
            x = x_next;
        }
        x
    }
}
