use crate::Solution;

fn permute_helper(mut nums: Vec<i32>, n: usize) -> Vec<Vec<i32>> {
    if let Some(cur) = nums.pop() {
        let mut ret: Vec<Vec<i32>> = Vec::new();
        for cur_perm in permute_helper(nums, n) {
            for p in 0..n {
                if cur_perm[p] == i32::MIN {
                    let mut clone = cur_perm.clone();
                    clone[p] = cur;
                    ret.push(clone);
                }
            }
        }
        ret

    } else {
        vec![vec![i32::MIN as i32; n]; 1]
    }
}

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let n = nums.len();
        permute_helper(nums, n)
    }
}
