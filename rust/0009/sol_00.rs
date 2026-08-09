use crate::Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false
        }

        let mut k = 1;
        let mut xx = x;
        while xx>=10 {
            k *= 10;
            xx /= 10;
        }

        let mut kk = 1;
        while k > kk {
            if (x / k) % 10 != (x / kk) % 10 {
                return false;
            }
            k /= 10;
            kk *= 10;
        }

        return true
    }
}
