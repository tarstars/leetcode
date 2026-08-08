#[path = "../sol_00.rs"]
mod sol_00;

struct Solution;

fn main() {
    let cases = [(vec![1, 3], vec![2]), (vec![1, 2], vec![3, 4])];

    for (a, b) in cases {
        println!(
            "{a:?} + {b:?} -> {}",
            Solution::find_median_sorted_arrays(a.clone(), b.clone())
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn median(a: &[i32], b: &[i32]) -> f64 {
        Solution::find_median_sorted_arrays(a.to_vec(), b.to_vec())
    }

    /// Compares with a tolerance rather than `==`, since a median is an average
    /// and floating point equality is a trap worth avoiding by habit.
    fn assert_close(got: f64, want: f64) {
        assert!(
            (got - want).abs() < 1e-9,
            "expected {want}, got {got}"
        );
    }

    #[test]
    fn example_1_odd_total_length() {
        assert_close(median(&[1, 3], &[2]), 2.0);
    }

    #[test]
    fn example_2_even_total_length() {
        assert_close(median(&[1, 2], &[3, 4]), 2.5);
    }

    #[test]
    fn first_array_is_empty() {
        assert_close(median(&[], &[1, 2, 3, 4]), 2.5);
    }

    #[test]
    fn second_array_is_empty() {
        assert_close(median(&[5], &[]), 5.0);
    }

    #[test]
    fn arrays_do_not_overlap() {
        assert_close(median(&[1, 2, 3], &[100, 200, 300]), 51.5);
    }

    #[test]
    fn one_array_is_much_longer() {
        assert_close(median(&[3], &[1, 2, 4, 5, 6, 7, 8, 9]), 5.0);
    }

    #[test]
    fn handles_duplicates() {
        assert_close(median(&[2, 2, 2], &[2, 2]), 2.0);
    }

    #[test]
    fn handles_negative_values() {
        assert_close(median(&[-5, -3], &[-2, -1]), -2.5);
    }

    #[test]
    fn single_element_each() {
        assert_close(median(&[1], &[2]), 1.5);
    }
}
