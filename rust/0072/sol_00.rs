use crate::Solution;

use std::cmp::min;

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let h = word1.len();
        let w = word2.len();

        let mut dp: Vec<Vec<i32>> = vec![vec![0; w + 1]; h + 1];

        for p in 1..=h {
            dp[p][0] = p as i32;
        }

        for q in 1..=w {
            dp[0][q] = q as i32;
        }

        for p in 1..=h {
            for q in 1..=w {
                dp[p][q] = min(
                    dp[p - 1][q - 1]
                        + if word1.as_bytes()[p - 1] == word2.as_bytes()[q - 1] {
                            0
                        } else {
                            1
                        },
                    min(dp[p - 1][q] + 1, dp[p][q - 1] + 1),
                );
            }
        }

        dp[h][w]
    }
}
