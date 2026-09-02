use crate::Solution;

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        s.split_ascii_whitespace()
            .next_back()
            .expect("s must contain at least one word")
            .len() as i32
    }
}
