use crate::Solution;

fn describe(sequence: &str) -> String {
    let mut result = String::new();
    let mut digits = sequence.chars().peekable();

    while let Some(digit) = digits.next() {
        let mut count = 1;

        while digits.next_if_eq(&digit).is_some() {
            count += 1;
        }

        result.push_str(&count.to_string());
        result.push(digit);
    }

    result
}

impl Solution {
    pub fn count_and_say(n: i32) -> String {
        (1..n).fold(String::from("1"), |sequence, _| describe(&sequence))
    }
}
