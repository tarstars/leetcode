#[path = "../sol_00.rs"]
mod sol_00;

struct Solution;

fn main() {
    for (s, words) in [
        ("barfoothefoobarman", vec!["foo", "bar"]),
        (
            "wordgoodgoodgoodbestword",
            vec!["word", "good", "best", "word"],
        ),
        ("barfoofoobarthefoobarman", vec!["bar", "foo", "the"]),
    ] {
        let words: Vec<String> = words.into_iter().map(String::from).collect();
        let indices = Solution::find_substring(s.to_string(), words.clone());
        println!("{s:?}, {words:?} -> {indices:?}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // The answer may be in any order, so compare sorted.
    fn find(s: &str, words: &[&str]) -> Vec<i32> {
        let words: Vec<String> = words.iter().map(|w| w.to_string()).collect();
        let mut indices = Solution::find_substring(s.to_string(), words);
        indices.sort();
        indices
    }

    fn reference(s: &str, words: &[&str]) -> Vec<i32> {
        let word_len = words[0].len();
        let total_len = word_len * words.len();
        let mut expected: Vec<&str> = words.to_vec();
        expected.sort();

        let mut indices = Vec::new();
        for start in 0..s.len().saturating_sub(total_len - 1) {
            let window = &s[start..start + total_len];
            let mut parts: Vec<&str> = (0..words.len())
                .map(|i| &window[i * word_len..(i + 1) * word_len])
                .collect();
            parts.sort();
            if parts == expected {
                indices.push(start as i32);
            }
        }
        indices
    }

    #[test]
    fn example_1() {
        assert_eq!(find("barfoothefoobarman", &["foo", "bar"]), vec![0, 9]);
    }

    #[test]
    fn example_2() {
        assert_eq!(
            find(
                "wordgoodgoodgoodbestword",
                &["word", "good", "best", "word"]
            ),
            vec![]
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            find("barfoofoobarthefoobarman", &["bar", "foo", "the"]),
            vec![6, 9, 12]
        );
    }

    #[test]
    fn repeated_words_must_be_counted() {
        // "good" must appear exactly twice in the window.
        assert_eq!(find("goodgoodbest", &["good", "good"]), vec![0]);
        assert_eq!(find("goodbestgood", &["good", "good"]), vec![]);
    }

    #[test]
    fn overlapping_matches() {
        assert_eq!(find("aaaaaa", &["aa", "aa"]), vec![0, 1, 2]);
    }

    #[test]
    fn single_word() {
        assert_eq!(find("abcabc", &["abc"]), vec![0, 3]);
    }

    #[test]
    fn single_characters() {
        assert_eq!(find("ab", &["a", "b"]), vec![0]);
        assert_eq!(find("ba", &["a", "b"]), vec![0]);
    }

    #[test]
    fn words_longer_than_s() {
        assert_eq!(find("ab", &["abc"]), vec![]);
        assert_eq!(find("ab", &["a", "b", "c"]), vec![]);
    }

    #[test]
    fn matches_reference_for_all_small_inputs() {
        // Every s of length 1..=8 over {a, b} against several word lists.
        let word_lists: Vec<Vec<&str>> = vec![
            vec!["a"],
            vec!["a", "b"],
            vec!["a", "a"],
            vec!["ab"],
            vec!["ab", "ba"],
            vec!["ab", "ab"],
            vec!["aab", "ba"],
        ];
        let mut strings = vec![String::new()];
        for _ in 0..8 {
            let longer: Vec<String> = strings
                .iter()
                .flat_map(|s| [format!("{s}a"), format!("{s}b")])
                .collect();
            strings.extend(longer);
        }
        for s in strings.iter().filter(|s| !s.is_empty()) {
            for words in &word_lists {
                if words.iter().map(|w| w.len()).sum::<usize>() > s.len() {
                    continue;
                }
                assert_eq!(
                    find(s, words),
                    reference(s, words),
                    "s: {s:?}, words: {words:?}"
                );
            }
        }
    }

    #[test]
    fn large_input() {
        // 10_000 characters, 30 words of length 30: brute force over every
        // start is ~9 million byte comparisons per window — a sliding-window
        // solution handles this easily, but so should any reasonable one.
        let words: Vec<String> = (0..30)
            .map(|i| {
                (0..30)
                    .map(|j| char::from(b'a' + ((i + j) % 26) as u8))
                    .collect()
            })
            .collect();
        let mut s = words.concat();
        s.push_str(&"z".repeat(10_000 - s.len()));
        let word_refs: Vec<&str> = words.iter().map(|w| w.as_str()).collect();
        let expected = reference(&s, &word_refs);
        assert!(expected.contains(&0));
        assert_eq!(find(&s, &word_refs), expected);
    }
}
