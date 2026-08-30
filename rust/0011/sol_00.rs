use crate::Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let n = height.len() as i32;
        let mut best_v = 0;

        let mut p: i32 = 0;
        let mut q: i32 = n - 1;

        while p < q {
            let v = std::cmp::min(height[p as usize], height[q as usize]) * (q - p);
            best_v = std::cmp::max(best_v, v);
            if height[p as usize] < height[q as usize] {
                p += 1;
            } else {
                q -= 1;
            }
        }

        best_v
    }
}
