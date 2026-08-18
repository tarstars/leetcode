#[path = "../sol_00.rs"]
mod sol_00;

struct Solution;

fn main() {
    for (haystack, needle) in [("sadbutsad", "sad"), ("leetcode", "leeto"), ("aaa", "aaaa")] {
        let index = Solution::str_str(haystack.to_string(), needle.to_string());
        println!("{haystack:?} / {needle:?} -> {index}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn find(haystack: &str, needle: &str) -> i32 {
        Solution::str_str(haystack.to_string(), needle.to_string())
    }

    fn reference(haystack: &str, needle: &str) -> i32 {
        haystack.find(needle).map_or(-1, |i| i as i32)
    }

    #[test]
    fn example_1() {
        assert_eq!(find("sadbutsad", "sad"), 0);
    }

    #[test]
    fn example_2() {
        assert_eq!(find("leetcode", "leeto"), -1);
    }

    #[test]
    fn needle_at_the_end() {
        assert_eq!(find("hello", "llo"), 2);
    }

    #[test]
    fn needle_equals_haystack() {
        assert_eq!(find("abc", "abc"), 0);
    }

    #[test]
    fn needle_longer_than_haystack() {
        assert_eq!(find("aaa", "aaaa"), -1);
    }

    #[test]
    fn single_characters() {
        assert_eq!(find("a", "a"), 0);
        assert_eq!(find("a", "b"), -1);
    }

    #[test]
    fn repetitive_near_miss() {
        // Naive scan must restart correctly after partial matches.
        assert_eq!(find("aaaaaaab", "aaab"), 4);
        assert_eq!(find("mississippi", "issip"), 4);
        assert_eq!(find("ababababac", "ababac"), 4);
    }

    #[test]
    fn matches_reference_for_all_small_inputs() {
        // Every haystack of length 1..=5 and needle of length 1..=3 over {a, b}.
        fn strings(max_len: usize) -> Vec<String> {
            let mut result = vec![String::new()];
            for _ in 0..max_len {
                let longer: Vec<String> = result
                    .iter()
                    .flat_map(|s| [format!("{s}a"), format!("{s}b")])
                    .collect();
                result.extend(longer);
            }
            result.into_iter().filter(|s| !s.is_empty()).collect()
        }
        for haystack in strings(5) {
            for needle in strings(3) {
                assert_eq!(
                    find(&haystack, &needle),
                    reference(&haystack, &needle),
                    "haystack: {haystack:?}, needle: {needle:?}"
                );
            }
        }
    }

    #[test]
    fn longest_allowed_input() {
        // Worst case for the naive algorithm: many long partial matches.
        let haystack = "a".repeat(10_000);
        let needle = format!("{}b", "a".repeat(4_999));
        assert_eq!(find(&haystack, &needle), -1);
        let needle = "a".repeat(5_000);
        assert_eq!(find(&haystack, &needle), 0);
    }
}
