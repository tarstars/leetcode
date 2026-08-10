#[path = "../sol_00.rs"]
mod sol_00;

struct Solution;

fn main() {
    for (nums, target) in [(vec![1, 0, -1, 0, -2, 2], 0), (vec![2, 2, 2, 2, 2], 8)] {
        println!(
            "{nums:?} target {target} -> {:?}",
            Solution::four_sum(nums.clone(), target)
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Any order is allowed, inside a quadruplet and between them.
    fn normalise(mut quads: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        for q in quads.iter_mut() {
            q.sort();
        }
        quads.sort();
        quads
    }

    fn four_sum(nums: &[i32], target: i32) -> Vec<Vec<i32>> {
        normalise(Solution::four_sum(nums.to_vec(), target))
    }

    fn expect(nums: &[i32], target: i32, want: &[[i32; 4]]) {
        let want = normalise(want.iter().map(|q| q.to_vec()).collect());
        assert_eq!(four_sum(nums, target), want, "for {nums:?}, target {target}");
    }

    #[test]
    fn example_1() {
        expect(
            &[1, 0, -1, 0, -2, 2],
            0,
            &[[-2, -1, 1, 2], [-2, 0, 0, 2], [-1, 0, 0, 1]],
        );
    }

    #[test]
    fn example_2_all_identical() {
        expect(&[2, 2, 2, 2, 2], 8, &[[2, 2, 2, 2]]);
    }

    /// A sixth 2 must not produce a second copy of the same quadruplet.
    #[test]
    fn extra_duplicates_change_nothing() {
        expect(&[2, 2, 2, 2, 2, 2], 8, &[[2, 2, 2, 2]]);
    }

    #[test]
    fn fewer_than_four_elements() {
        expect(&[1, 2, 3], 6, &[]);
        expect(&[0], 0, &[]);
    }

    #[test]
    fn four_zeros() {
        expect(&[0, 0, 0, 0], 0, &[[0, 0, 0, 0]]);
    }

    #[test]
    fn no_quadruplet_sums_to_target() {
        expect(&[1, 2, 3, 4, 5], 100, &[]);
    }

    #[test]
    fn symmetric_input() {
        expect(
            &[-3, -2, -1, 0, 0, 1, 2, 3],
            0,
            &[
                [-3, -2, 2, 3],
                [-3, -1, 1, 3],
                [-3, 0, 0, 3],
                [-3, 0, 1, 2],
                [-2, -1, 0, 3],
                [-2, -1, 1, 2],
                [-2, 0, 0, 2],
                [-1, 0, 0, 1],
            ],
        );
    }

    /// Values at both ends of the allowed range: the sum of the two negatives
    /// is -2 * 10^9, which alone is outside i32 when added to anything positive
    /// in the wrong order.
    #[test]
    fn extreme_values() {
        expect(
            &[-1_000_000_000, -1_000_000_000, 1_000_000_000, 1_000_000_000],
            0,
            &[[-1_000_000_000, -1_000_000_000, 1_000_000_000, 1_000_000_000]],
        );
    }

    /// Four times 10^9 is 4*10^9, which wraps to exactly -294967296 in i32.
    /// Accumulating the sum in i32 therefore reports a match that is not one
    /// (or panics in debug builds). The answer is empty.
    #[test]
    fn sum_overflows_i32() {
        expect(&[1_000_000_000; 4], -294_967_296, &[]);
    }

    /// 200 elements — the constraint's upper bound — with 2171 quadruplets.
    #[test]
    fn large_input_with_many_hits() {
        let nums: Vec<i32> = (0..200).map(|i: i64| ((i * i * 13) % 401 - 200) as i32).collect();
        let got = normalise(Solution::four_sum(nums, 601));

        assert_eq!(got.len(), 2171);
        assert_eq!(got[0], vec![19, 189, 195, 198]);
        assert_eq!(got[got.len() - 1], vec![142, 147, 153, 159]);

        for q in &got {
            assert_eq!(q.iter().map(|&x| x as i64).sum::<i64>(), 601);
        }
    }

    /// Same array, a target nothing reaches — the search must still finish fast.
    #[test]
    fn large_input_with_no_hits() {
        let nums: Vec<i32> = (0..200).map(|i: i64| ((i * i * 13) % 401 - 200) as i32).collect();
        assert!(Solution::four_sum(nums, 7919).is_empty());
    }
}
