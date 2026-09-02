use crate::Solution;

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let last_char = (0..s.len()).rfind(|p|s.as_bytes()[*p].is_ascii_alphabetic());
        if let Some(last_char) = last_char {
            let pre_char = (0..last_char).rfind(|p|!s.as_bytes()[*p].is_ascii_alphabetic());
            return pre_char.map_or(last_char + 1, |x| last_char - x) as i32
        }
        return 0;
    }
}
