use crate::Solution;

use std::cmp::max;

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut st: Vec<i32> = Vec::new();
        let mut max_val: i32 = 0;
        st.push(-1);

        for (pos, c) in s.chars().enumerate() {
            match c {
                '(' => {
                    st.push(pos as i32);
                }
                ')' => {
                    st.pop();
                    if st.is_empty() {
                        st.push(pos as i32);
                    }
                    let lpos = *st.last().unwrap();
                    max_val = max(max_val, pos as i32 - lpos);
                }
                _ => {}
            }
        }

        max_val
    }
}
