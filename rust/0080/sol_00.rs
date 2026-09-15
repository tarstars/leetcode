use crate::Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut prev: Option<i32> = None;
        let mut pcount: u8 = 0;
        let mut p_write: usize = 0;
        let n = nums.len();

        for p in 0..n {
            match prev {
                Some(pval) => {
                    if nums[p] != pval {
                        prev = Some(nums[p]);
                        nums[p_write] = nums[p];
                        pcount = 1;
                        p_write += 1;
                    } else {
                        if pcount < 2 {
                            nums[p_write] = nums[p];
                            pcount += 1;
                            p_write += 1;
                        }
                    }
                }
                _ => {
                    prev = Some(nums[p]);
                    pcount = 1;
                    p_write = 1;
                }
            }
        }
        p_write as i32

    }
}
