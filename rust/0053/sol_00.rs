use crate::Solution;

use std::cmp::max;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut s: Option<i32> = None;
        let mut max_s: Option<i32> = None;

        for &v in &nums {
            s = if s.is_none() {Some(v)} else {Some(max(s.unwrap() + v, v))};
            max_s = if max_s.is_none() {Some(v)} else {Some(max(s.unwrap(), max_s.unwrap()))};
        }

        max_s.unwrap()
    }
}
