use crate::Solution;

fn c(s: &[u8], p: i32) -> u8 {
    if p % 2 == 1 {
        return s[(p / 2) as usize]
    }
    b'#'
}

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let n: i32 = (s.len()*2 + 1) as i32;
        let b = s.as_bytes();

        let mut bestCenter: i32 = 0;
        let mut bestLength: i32 = 0;

        for p in 0i32..n {
            let mut r: i32 = 0;
            while p - r - 1  >= 0 && p + r + 1 < n && c(b, p - r - 1) == c(b, p + r + 1) {
                r = r + 1;
            }
            if r > bestLength {
                bestLength = r;
                bestCenter = p;
            } 
        }

        let start = ((bestCenter - bestLength) / 2) as usize;
        s[start..start + bestLength as usize].to_string()
    }
}
