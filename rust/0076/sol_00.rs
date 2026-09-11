use crate::Solution;

use std::collections::HashMap;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let s: Vec<char> = s.chars().collect();
        let mut window_count: HashMap<char, i32> = HashMap::new();
        let mut best_length = i32::MAX;
        let mut best_left = 0;
        let mut best_right = 0;

        for c in t.chars() {
            *window_count.entry(c).or_insert(0) += 1;
        }

        let mut pos = window_count.len();
        let mut left = 0;
        for (right, c) in s.iter().enumerate() {
            if let Some(count) = window_count.get_mut(&c) {
                *count -= 1;
                if *count == 0 {
                    pos -= 1;
                }
            }
            while pos == 0 {
                if right - left + 1 < best_length as usize {
                    best_length = (right - left + 1) as i32;
                    best_left = left;
                    best_right = right;
                }
                if let Some(count) = window_count.get_mut(&s[left]) {
                    *count += 1;
                    if *count == 1 {
                        pos += 1;
                    }
                }
                left += 1;
            }
        }

        if best_length == i32::MAX {
            "".to_string()
        } else {
            s[best_left..=best_right].iter().collect()
        }
    }
}
