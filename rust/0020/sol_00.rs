use crate::Solution;

use std::collections::HashMap;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let b:HashMap<char, i8> = HashMap::from([
            ('{', -1i8),
            ('}', 1),
            ('(', -2),
            (')', 2),
            ('[', -3),
            (']', 3)]);
        
        let mut st:Vec<i8> = Vec::new();
        for c in s.chars() {
            let cv = *b.get(&c).unwrap();
            if cv < 0i8 {
                st.push(cv);
            } else {
                match st.pop() {
                    Some(top) if top + cv == 0 => {}
                    _ => return false,
                }
            }
        }

        return st.len() == 0;
    }
}
