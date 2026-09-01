use crate::Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut values = nums.into_iter();
        let first = values.next().expect("nums must be non-empty");

        values
            .fold((first, first), |(current_sum, maximum_sum), value| {
                let current_sum = value.max(current_sum + value);
                (current_sum, maximum_sum.max(current_sum))
            })
            .1
    }
}
