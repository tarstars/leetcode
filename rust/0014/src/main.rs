#[path = "../sol_00.rs"]
mod sol_00;

struct Solution;

fn main() {
    for strs in [vec!["flower", "flow", "flight"], vec!["dog", "racecar", "car"]] {
        let owned: Vec<String> = strs.iter().map(|s| s.to_string()).collect();
        println!("{strs:?} -> {:?}", Solution::longest_common_prefix(owned));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lcp(strs: &[&str]) -> String {
        Solution::longest_common_prefix(strs.iter().map(|s| s.to_string()).collect())
    }

    #[test]
    fn example_1() {
        assert_eq!(lcp(&["flower", "flow", "flight"]), "fl");
    }

    #[test]
    fn example_2_no_common_prefix() {
        assert_eq!(lcp(&["dog", "racecar", "car"]), "");
    }

    #[test]
    fn single_string_is_its_own_prefix() {
        assert_eq!(lcp(&["alone"]), "alone");
    }

    #[test]
    fn identical_strings() {
        assert_eq!(lcp(&["same", "same", "same"]), "same");
    }

    /// The shortest string caps the answer, wherever it sits in the list.
    #[test]
    fn shortest_string_is_the_whole_prefix() {
        assert_eq!(lcp(&["interspecies", "inter", "interstellar"]), "inter");
        assert_eq!(lcp(&["ab", "abc", "abcd"]), "ab");
    }

    /// An empty string anywhere forces an empty answer.
    #[test]
    fn empty_string_in_the_list() {
        assert_eq!(lcp(&["", "b"]), "");
        assert_eq!(lcp(&["a", ""]), "");
        assert_eq!(lcp(&["abc", "abc", ""]), "");
    }

    #[test]
    fn only_an_empty_string() {
        assert_eq!(lcp(&[""]), "");
    }

    #[test]
    fn differs_at_the_first_character() {
        assert_eq!(lcp(&["apple", "banana"]), "");
    }

    #[test]
    fn one_character_prefix() {
        assert_eq!(lcp(&["a", "ab", "abc"]), "a");
    }

    /// The mismatch is in the last pair checked, not the first.
    #[test]
    fn mismatch_in_the_final_string() {
        assert_eq!(lcp(&["prefix", "prefix", "prefix", "prefiy"]), "prefi");
    }

    /// 200 strings of 200 characters — the constraint's upper bound.
    #[test]
    fn many_long_strings() {
        let base = "a".repeat(200);
        let mut strs: Vec<String> = vec![base.clone(); 199];
        strs.push(format!("{}b", "a".repeat(199)));
        assert_eq!(
            Solution::longest_common_prefix(strs),
            "a".repeat(199)
        );
    }
}
