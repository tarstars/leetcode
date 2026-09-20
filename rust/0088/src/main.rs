#[path = "../sol_00.rs"]
mod sol_00;

struct Solution;

fn main() {
    for (mut nums1, m, nums2, n) in [
        (vec![1, 2, 3, 0, 0, 0], 3, vec![2, 5, 6], 3),
        (vec![1], 1, vec![], 0),
        (vec![0], 0, vec![1], 1),
    ] {
        let before = nums1.clone();
        Solution::merge(&mut nums1, m, nums2, n);
        println!("{before:?} -> {nums1:?}");
    }
}

#[cfg(test)]
mod tests;
