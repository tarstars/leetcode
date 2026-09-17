use crate::Solution;

use std::cmp::max;

impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut largest_area = 0;
        let mut st: Vec<(i32, i32)> = Vec::new();

        for (pos, v) in heights.iter().enumerate() {
            while !st.is_empty() && st.last().unwrap().1 > *v {
                let (ppos, pv) = st.pop().unwrap();
                match st.last() {
                    Some((pppos, ppv)) => {
                        largest_area = max(largest_area, (pos as i32 - pppos - 1) * pv);
                    }
                    _ => {
                        largest_area = max(largest_area, pv as i32 * pos as i32);
                    }
                }
            }
            st.push((pos as i32, *v));
        }

        while !st.is_empty() {
            let (pos, v) = st.pop().unwrap();
            match st.last() {
                Some((ppos, pv)) => {
                    largest_area = max(largest_area, (heights.len() as i32 - ppos - 1) * v);
                }
                _ => {
                    largest_area = max(largest_area, v * heights.len() as i32);
                }
            }
        }

        largest_area
    }
}
