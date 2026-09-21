use crate::Solution;

impl Solution {
    pub fn gray_code(n: i32) -> Vec<i32> {
        (0..(1 << n)).map(|x| x ^ (x >> 1)).collect()
    }
}
