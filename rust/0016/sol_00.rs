use crate::Solution;

impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        let n = nums.len();
        nums.sort();
        let mut best_sum: Option<i32> = None;

        for q in 1..n-1 {
            let mut p = 0;
            let mut r = n - 1;
            
            loop {
                let t = nums[p] + nums[q] + nums[r];

                if best_sum == None || (t - target).abs() < (best_sum.unwrap() - target).abs() {
                    best_sum = Some(t);
                }

                if t > target {
                    if r - 1 > q {
                        r -= 1;
                        continue;
                    }
                    break;                    
                }
                if t < target {
                    if p + 1 < q {
                        p += 1;
                        continue;
                    }
                    break;
                }

                if p + 1 < q {
                    p += 1;
                    continue;
                } 

                if r - 1 > q {
                    r -= 1;
                    continue;
                }
                break;
            }

        }

        best_sum.unwrap() 
    }
}
