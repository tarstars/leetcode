use crate::Solution;

use std::collections::HashSet;

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let n = s.len();
        let mut dp: Vec<i32> = vec![0; n + 1];
        let s: Vec<char> = s.chars().collect();
        let mut good: HashSet<String> = HashSet::new();

        for p in 1..=26 {
            good.insert(p.to_string());
        }

        dp[0] = 1;

        for p in 1i32..=n as i32 {
            if good.contains(&s[p as usize - 1..p as usize].iter().collect::<String>()) {
                dp[p as usize] += dp[p as usize - 1];
            }
            if p - 2 >= 0
                && good.contains(&s[p as usize - 2..p as usize].iter().collect::<String>())
            {
                dp[p as usize] += dp[p as usize - 2];
            }
        }

        dp[n]
    }
}
