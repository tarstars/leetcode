#[path = "../sol_00.rs"]
mod sol_00;

struct Solution;

fn main() {
    for h in [vec![1, 8, 6, 2, 5, 4, 8, 3, 7], vec![1, 1], vec![4, 3, 2, 1, 4]] {
        println!("{h:?} -> {}", Solution::max_area(h.clone()));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn max_area(h: &[i32]) -> i32 {
        Solution::max_area(h.to_vec())
    }

    #[test]
    fn example_1() {
        assert_eq!(max_area(&[1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    }

    #[test]
    fn example_2_two_lines() {
        assert_eq!(max_area(&[1, 1]), 1);
    }

    /// The best pair is the outer one, not the tallest neighbours.
    #[test]
    fn widest_pair_wins() {
        assert_eq!(max_area(&[4, 3, 2, 1, 4]), 16);
    }

    #[test]
    fn three_lines() {
        assert_eq!(max_area(&[1, 2, 1]), 2);
    }

    /// The best pair is the tall adjacent one, not any wide pair.
    #[test]
    fn tallest_adjacent_pair_wins() {
        assert_eq!(max_area(&[2, 3, 4, 5, 18, 17, 6]), 17);
    }

    #[test]
    fn zero_heights_hold_nothing() {
        assert_eq!(max_area(&[0, 0]), 0);
        assert_eq!(max_area(&[0, 2]), 0);
    }

    #[test]
    fn monotonic_increasing() {
        assert_eq!(max_area(&[1, 2, 3, 4, 5]), 6);
    }

    #[test]
    fn monotonic_decreasing() {
        assert_eq!(max_area(&[5, 4, 3, 2, 1]), 6);
    }

    #[test]
    fn tall_ends_flat_middle() {
        assert_eq!(max_area(&[100, 0, 0, 0, 100]), 400);
    }

    /// 100_000 lines: the constraint's upper bound. An O(n^2) scan is ~5*10^9
    /// steps and will not finish in any reasonable time. The answer here is
    /// 998_100_000 — within i32, but close enough to its 2.1*10^9 limit to be
    /// worth noticing.
    #[test]
    fn large_input_needs_linear_time() {
        let h: Vec<i32> = (0..100_000).map(|i: i64| ((i * i) % 10_001) as i32).collect();
        assert_eq!(Solution::max_area(h), 998_100_000);
    }
}
