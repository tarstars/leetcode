use super::*;

fn justify(words: &[&str], max_width: i32) -> Vec<String> {
    let owned: Vec<String> = words.iter().map(|w| (*w).to_owned()).collect();
    Solution::full_justify(owned, max_width)
}

fn justify_text(text: &str, max_width: i32) -> Vec<String> {
    let owned: Vec<String> = text.split_whitespace().map(str::to_owned).collect();
    Solution::full_justify(owned, max_width)
}

/// Checks the output against the statement's rules directly, rather than
/// against another implementation. The rules pin the answer uniquely, so this
/// is a complete oracle: greedy packing fixes which words share a line, and the
/// spacing rules fix each line's content given that partition.
fn assert_justified(words: &[&str], max_width: usize, lines: &[String]) {
    assert!(!lines.is_empty(), "no lines produced for {words:?}");

    // Every line is exactly max_width characters.
    for (i, line) in lines.iter().enumerate() {
        assert_eq!(
            line.chars().count(),
            max_width,
            "line {i} of {words:?} is {:?}, width {} not {max_width}",
            line,
            line.chars().count()
        );
    }

    // Recovering the words from the output reproduces the input, in order.
    let recovered: Vec<&str> = lines
        .iter()
        .flat_map(|line| line.split_whitespace())
        .collect();
    assert_eq!(recovered, words, "words were lost, reordered or altered");

    // Split the input back into the per-line groups the output implies.
    let mut groups: Vec<Vec<&str>> = Vec::new();
    for line in lines {
        groups.push(line.split_whitespace().collect());
    }

    // Greedy: each line but the last is full — the first word of the next line
    // could not have been appended to it.
    for (i, group) in groups.iter().enumerate() {
        assert!(!group.is_empty(), "line {i} of {words:?} holds no word");

        if let Some(next) = groups.get(i + 1).and_then(|g| g.first()) {
            let used: usize = group.iter().map(|w| w.len()).sum::<usize>() + group.len() - 1;
            assert!(
                used + 1 + next.len() > max_width,
                "line {i} of {words:?} is not greedy: {:?} had room for {next:?}",
                lines[i]
            );
        }
    }

    // Each line's spacing.
    let last = groups.len() - 1;
    for (i, group) in groups.iter().enumerate() {
        let letters: usize = group.iter().map(|w| w.len()).sum();

        let expected = if i == last || group.len() == 1 {
            // Left-justified: single spaces, then padding on the right.
            let joined = group.join(" ");
            format!("{joined}{}", " ".repeat(max_width - joined.len()))
        } else {
            // Fully justified: extra spaces spread left-heavy.
            let gaps = group.len() - 1;
            let total = max_width - letters;
            let base = total / gaps;
            let extra = total % gaps;

            let mut line = String::from(group[0]);
            for (g, word) in group[1..].iter().enumerate() {
                let width = base + usize::from(g < extra);
                line.push_str(&" ".repeat(width));
                line.push_str(word);
            }
            line
        };

        assert_eq!(&lines[i], &expected, "line {i} of {words:?}");
    }
}

/// Deterministic word lists, so any failure reproduces.
struct Rng(u64);

impl Rng {
    fn next_value(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }

    fn below(&mut self, bound: u64) -> usize {
        (self.next_value() % bound) as usize
    }

    fn words(&mut self, count: usize, max_len: usize) -> Vec<String> {
        (0..count)
            .map(|_| {
                let len = 1 + self.below(max_len as u64);
                (0..len)
                    .map(|_| (b'a' + self.below(26) as u8) as char)
                    .collect()
            })
            .collect()
    }
}

#[test]
fn example_1() {
    assert_eq!(
        justify_text("This is an example of text justification.", 16),
        vec!["This    is    an", "example  of text", "justification.  "]
    );
}

/// The second line holds one word, so it is left-justified rather than
/// stretched; the last line is left-justified too.
#[test]
fn example_2() {
    assert_eq!(
        justify_text("What must be acknowledgment shall be", 16),
        vec!["What   must   be", "acknowledgment  ", "shall be        "]
    );
}

#[test]
fn example_3() {
    assert_eq!(
        justify_text(
            "Science is what we understand well enough to explain to a computer. \
             Art is everything else we do",
            20
        ),
        vec![
            "Science  is  what we",
            "understand      well",
            "enough to explain to",
            "a  computer.  Art is",
            "everything  else  we",
            "do                  ",
        ]
    );
}

/// One word is always the last line, so it is left-justified and padded.
#[test]
fn a_single_word() {
    assert_eq!(justify(&["a"], 1), vec!["a"]);
    assert_eq!(justify(&["a"], 5), vec!["a    "]);
    assert_eq!(justify(&["hello"], 5), vec!["hello"]);
    assert_eq!(justify(&["hello"], 9), vec!["hello    "]);
}

/// A word exactly as wide as the line gets a line to itself.
#[test]
fn a_word_that_fills_the_line() {
    assert_eq!(justify(&["abcde", "fg"], 5), vec!["abcde", "fg   "]);
    assert_eq!(justify(&["fg", "abcde"], 5), vec!["fg   ", "abcde"]);
}

/// Uneven space distribution must favour the left gaps. The trailing long word
/// is there only to force a wrap, so the first line is not the last one.
#[test]
fn extra_spaces_go_to_the_left() {
    // 4 letters over 2 gaps at width 9: 5 spaces to spread, so 3 then 2.
    assert_eq!(
        justify(&["a", "bb", "c", "dddddddd"], 9),
        vec!["a   bb  c", "dddddddd "]
    );
    // 3 letters over 2 gaps at width 8: 5 spaces to spread, so 3 then 2.
    assert_eq!(
        justify(&["a", "b", "c", "ddddddd"], 8),
        vec!["a   b  c", "ddddddd "]
    );
}

/// Every line but the last holds exactly one word, so none is stretched.
#[test]
fn every_word_on_its_own_line() {
    assert_eq!(
        justify(&["aaaa", "bbbb", "cccc"], 5),
        vec!["aaaa ", "bbbb ", "cccc "]
    );
}

/// A width of one admits only single-character words, one per line.
#[test]
fn width_of_one() {
    assert_eq!(justify(&["a", "b", "c"], 1), vec!["a", "b", "c"]);
}

/// The whole input fitting on one line makes it the last line, so it is
/// left-justified rather than stretched.
#[test]
fn everything_on_one_line_is_left_justified() {
    assert_eq!(justify(&["a", "b"], 10), vec!["a b       "]);
    assert_eq!(
        justify(&["one", "two", "three"], 20),
        vec!["one two three       "]
    );
}

/// The statement's own examples, checked through the rule validator rather than
/// against fixed strings — this confirms the validator agrees with the
/// statement before it is trusted on generated input.
#[test]
fn the_validator_accepts_the_statement_examples() {
    let words: Vec<&str> = "This is an example of text justification."
        .split_whitespace()
        .collect();
    let lines = vec![
        "This    is    an".to_owned(),
        "example  of text".to_owned(),
        "justification.  ".to_owned(),
    ];
    assert_justified(&words, 16, &lines);

    let words: Vec<&str> = "What must be acknowledgment shall be"
        .split_whitespace()
        .collect();
    let lines = vec![
        "What   must   be".to_owned(),
        "acknowledgment  ".to_owned(),
        "shall be        ".to_owned(),
    ];
    assert_justified(&words, 16, &lines);
}

/// Every rule, on a wide spread of shapes: word counts, word lengths and
/// widths chosen to exercise both tight and roomy lines.
#[test]
fn obeys_every_rule_on_generated_input() {
    let mut rng = Rng(0x9E37_79B9_7F4A_7C15);

    for max_width in [1usize, 2, 3, 5, 8, 13, 20, 47, 100] {
        for count in [1usize, 2, 3, 7, 20, 100] {
            let max_len = max_width.min(20);
            let owned = rng.words(count, max_len);
            let words: Vec<&str> = owned.iter().map(String::as_str).collect();

            let lines = justify(&words, max_width as i32);
            assert_justified(&words, max_width, &lines);
        }
    }
}

/// Many equal-length words, which is where an off-by-one in the gap
/// arithmetic shows up most readily.
#[test]
fn uniform_word_lengths() {
    for word_len in 1..=6usize {
        for max_width in word_len..=20 {
            let word = "x".repeat(word_len);
            let owned = vec![word; 17];
            let words: Vec<&str> = owned.iter().map(String::as_str).collect();

            let lines = justify(&words, max_width as i32);
            assert_justified(&words, max_width, &lines);
        }
    }
}

/// The constraints' maximum: 300 words of up to 20 characters at width 100.
#[test]
fn largest_allowed_input() {
    let mut rng = Rng(0xDEAD_BEEF_CAFE_F00D);
    let owned = rng.words(300, 20);
    let words: Vec<&str> = owned.iter().map(String::as_str).collect();

    let lines = justify(&words, 100);
    assert_justified(&words, 100, &lines);

    // 300 single characters at width 100 pack densely: 50 per line.
    let owned = vec!["z".to_owned(); 300];
    let words: Vec<&str> = owned.iter().map(String::as_str).collect();
    let lines = justify(&words, 100);
    assert_justified(&words, 100, &lines);
    assert_eq!(lines.len(), 6);
}
