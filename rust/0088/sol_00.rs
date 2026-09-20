use crate::Solution;

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: Vec<i32>, n: i32) {
        let mut z = nums1.len() as i32 - 1;
        let mut x = m - 1;
        let mut y = n - 1;

        while x >= 0 && y >= 0 {
            if nums1[x as usize] > nums2[y as usize] {
                nums1[z as usize] = nums1[x as usize];
                x -= 1;
                z -= 1;
            } else {
                nums1[z as usize] = nums2[y as usize];
                y -= 1;
                z -= 1;
            }
        }

        while y >= 0 {
            nums1[z as usize] = nums2[y as usize];
            z -= 1;
            y -= 1;
        }
    }
}
