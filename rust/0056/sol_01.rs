use crate::Solution;

impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        intervals.sort_unstable_by_key(|interval| interval[0]);

        let mut merged: Vec<Vec<i32>> = Vec::with_capacity(intervals.len());
        for interval in intervals {
            match merged.last_mut() {
                Some(previous) if interval[0] <= previous[1] => {
                    previous[1] = previous[1].max(interval[1]);
                }
                _ => merged.push(interval),
            }
        }

        merged
    }
}
