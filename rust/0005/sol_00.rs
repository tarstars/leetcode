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

        let mut best_center: i32 = 0;
        let mut best_length: i32 = 0;

        for p in 0i32..n {
            let mut r: i32 = 0;
            while p - r - 1  >= 0 && p + r + 1 < n && c(b, p - r - 1) == c(b, p + r + 1) {
                r = r + 1;
            }
            if r > best_length {
                best_length = r;
                best_center = p;
            } 
        }

        let start = ((best_center - best_length) / 2) as usize;
        s[start..start + best_length as usize].to_string()
    }
}
