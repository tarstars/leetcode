use crate::Solution;
use std::collections::HashSet;

impl Solution {
    pub fn length_of_longest_substring(ss: String) -> i32 {
        let s = ss.as_bytes();
        let n = s.len();
        let mut q: usize = 0;
        let mut windowChars: HashSet<u8> = std::collections::HashSet::new();
        let mut maxAns: i32 = 0;

        for p in 0..n {
            while q < n && !windowChars.contains(&s[q]) {
                windowChars.insert(s[q]);
                q+=1;
            }
            if windowChars.len() as i32 > maxAns {
                maxAns = windowChars.len() as i32;
            }
            windowChars.remove(&s[p]);
        }

        maxAns
    }
}
