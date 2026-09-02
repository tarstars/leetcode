use crate::Solution;

use std::cmp::max;

impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        intervals.sort_unstable_by_key(|v|v[0]);
        let mut simplified_intervals: Vec<Vec<i32>> = Vec::new();
        let mut it = intervals.into_iter();

        let mut cand = it.next().unwrap();

        for element in it {
            if element[0] <= cand[1] {
                cand[1] = max(cand[1], element[1]);
            } else {
                simplified_intervals.push(cand);
                cand = element;
            }
        }

        simplified_intervals.push(cand);

        simplified_intervals
    }
}
