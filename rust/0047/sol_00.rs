use crate::Solution;

fn permute_helper(mut nums: Vec<i32>, n: usize) -> Vec<Vec<i32>> {
    let mut all_perms : Vec<Vec<i32>> = Vec::new();

    if let Some(digit) = nums.pop() {
        for perm in permute_helper(nums, n) {
            let offset = (0..n).rfind(|ind| perm[*ind] == digit);
            for pos in offset.map_or(0, |x| x + 1)..n {
                if perm[pos] != i32::MIN {
                    continue;
                }
                let mut new_perm = perm.clone();
                new_perm[pos] = digit;
                all_perms.push(new_perm);
            }
        }
    } else {
        all_perms.push(vec![i32::MIN; n]);
    }

    all_perms
}

impl Solution {
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let n = nums.len();
        permute_helper(nums, n)
    }
}
