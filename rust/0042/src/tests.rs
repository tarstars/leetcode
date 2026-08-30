use super::*;

#[test]
fn example_1() {
    assert_eq!(Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
}

#[test]
fn example_2() {
    assert_eq!(Solution::trap(vec![4, 2, 0, 3, 2, 5]), 9);
}

#[test]
fn short_maps_cannot_trap_water() {
    assert_eq!(Solution::trap(vec![7]), 0);
    assert_eq!(Solution::trap(vec![7, 1]), 0);
}

#[test]
fn monotone_maps_cannot_trap_water() {
    assert_eq!(Solution::trap(vec![0, 1, 2, 3, 4]), 0);
    assert_eq!(Solution::trap(vec![4, 3, 2, 1, 0]), 0);
}

#[test]
fn traps_water_in_a_simple_bowl() {
    assert_eq!(Solution::trap(vec![3, 0, 3]), 3);
}

#[test]
fn uses_the_shorter_boundary() {
    assert_eq!(Solution::trap(vec![5, 0, 1, 0, 2]), 5);
}

#[test]
fn handles_multiple_basins() {
    assert_eq!(Solution::trap(vec![2, 0, 2, 0, 2]), 4);
}

#[test]
fn handles_the_maximum_map_size() {
    let mut height = vec![0; 20_000];
    height[0] = 100_000;
    height[19_999] = 100_000;
    assert_eq!(Solution::trap(height), 1_999_800_000);
}
