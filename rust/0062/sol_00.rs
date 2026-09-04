use crate::Solution;

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let m = m - 1;
        let n = n - 1;
        let mut c = 1;

        for q in 0..m {
            c = c * (q + n + 1) / (q + 1); 
        }

        c
    }
}
