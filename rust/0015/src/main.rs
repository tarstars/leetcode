// Two solutions share one test suite. `cargo test` runs sol_00;
// `cargo test --features sol_01` runs sol_01.
#[cfg(not(feature = "sol_01"))]
#[path = "../sol_00.rs"]
mod sol_00;

#[cfg(feature = "sol_01")]
#[path = "../sol_01.rs"]
mod sol_01;

struct Solution;

fn main() {
    for nums in [vec![-1, 0, 1, 2, -1, -4], vec![0, 1, 1], vec![0, 0, 0]] {
        println!("{nums:?} -> {:?}", Solution::three_sum(nums.clone()));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The problem says neither the order of the triplets nor the order within
    /// a triplet matters, so both are normalised before comparing.
    fn normalise(mut triplets: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        for t in triplets.iter_mut() {
            t.sort();
        }
        triplets.sort();
        triplets
    }

    fn three_sum(nums: &[i32]) -> Vec<Vec<i32>> {
        normalise(Solution::three_sum(nums.to_vec()))
    }

    fn expect(nums: &[i32], want: &[[i32; 3]]) {
        let want = normalise(want.iter().map(|t| t.to_vec()).collect());
        assert_eq!(three_sum(nums), want, "for input {nums:?}");
    }

    #[test]
    fn example_1() {
        expect(&[-1, 0, 1, 2, -1, -4], &[[-1, -1, 2], [-1, 0, 1]]);
    }

    #[test]
    fn example_2_no_triplet() {
        expect(&[0, 1, 1], &[]);
    }

    #[test]
    fn example_3_all_zeros() {
        expect(&[0, 0, 0], &[[0, 0, 0]]);
    }

    /// A fourth zero must not produce a second copy of [0,0,0].
    #[test]
    fn extra_zero_is_not_a_new_triplet() {
        expect(&[0, 0, 0, 0], &[[0, 0, 0]]);
    }

    #[test]
    fn no_triplet_when_all_positive() {
        expect(&[1, 1, 1], &[]);
    }

    #[test]
    fn repeated_value_used_twice() {
        expect(&[-2, 0, 1, 1, 2], &[[-2, 0, 2], [-2, 1, 1]]);
        expect(&[-1, -1, 2, 2, -4], &[[-4, 2, 2], [-1, -1, 2]]);
    }

    #[test]
    fn input_order_does_not_matter() {
        expect(&[3, 0, -2, -1, 1, 2], &[[-2, -1, 3], [-2, 0, 2], [-1, 0, 1]]);
    }

    /// Many duplicates on both sides — the usual place a dedup rule leaks.
    #[test]
    fn many_duplicates() {
        expect(
            &[-4, -2, -2, -2, 0, 1, 2, 2, 2, 3, 3, 4, 4, 6, 6],
            &[
                [-4, -2, 6],
                [-4, 0, 4],
                [-4, 1, 3],
                [-4, 2, 2],
                [-2, -2, 4],
                [-2, 0, 2],
            ],
        );
    }

    #[test]
    fn minimum_length_input() {
        expect(&[-1, 0, 1], &[[-1, 0, 1]]);
        expect(&[1, 2, 3], &[]);
    }

    /// 3000 identical zeros: one triplet, but a cubic scan would examine
    /// 4.5 * 10^9 index combinations getting there.
    #[test]
    fn three_thousand_zeros() {
        assert_eq!(Solution::three_sum(vec![0; 3000]), vec![vec![0, 0, 0]]);
    }

    /// 3000 values at the constraint's upper bound, holding 201 distinct values
    /// and yielding 2440 distinct triplets.
    #[test]
    fn large_input_needs_quadratic_time() {
        let nums: Vec<i32> = (0..3000).map(|i: i64| ((i * i) % 401 - 200) as i32).collect();
        let got = normalise(Solution::three_sum(nums));

        assert_eq!(got.len(), 2440);
        assert_eq!(got[0], vec![-200, 0, 200]);
        assert_eq!(got[got.len() / 2], vec![-123, -27, 150]);
        assert_eq!(got[got.len() - 1], vec![0, 0, 0]);

        for t in &got {
            assert_eq!(t.iter().sum::<i32>(), 0, "{t:?} does not sum to zero");
        }
    }
}
