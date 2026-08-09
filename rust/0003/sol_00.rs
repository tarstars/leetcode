use crate::Solution;
use std::collections::HashSet;

impl Solution {
    pub fn length_of_longest_substring(ss: String) -> i32 {
        let s = ss.as_bytes();
        let n = s.len();
        let mut q: usize = 0;
        let mut window_chars: HashSet<u8> = std::collections::HashSet::new();
        let mut max_ans: i32 = 0;

        for p in 0..n {
            while q < n && !window_chars.contains(&s[q]) {
                window_chars.insert(s[q]);
                q+=1;
            }
            if window_chars.len() as i32 > max_ans {
                max_ans = window_chars.len() as i32;
            }
            window_chars.remove(&s[p]);
        }

        max_ans
    }
}
