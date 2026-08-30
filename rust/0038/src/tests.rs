use super::*;

fn encode(value: &str) -> String {
    let mut result = String::new();
    let mut digits = value.chars().peekable();

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

#[test]
fn example_1() {
    assert_eq!(Solution::count_and_say(4), "1211");
}

#[test]
fn example_2() {
    assert_eq!(Solution::count_and_say(1), "1");
}

#[test]
fn generates_the_first_five_values() {
    let expected = ["1", "11", "21", "1211", "111221"];

    for (index, expected) in expected.into_iter().enumerate() {
        assert_eq!(Solution::count_and_say(index as i32 + 1), expected);
    }
}

#[test]
fn every_value_describes_the_previous_value() {
    let mut previous = Solution::count_and_say(1);

    for n in 2..=30 {
        let current = Solution::count_and_say(n);
        assert_eq!(current, encode(&previous), "n: {n}");
        previous = current;
    }
}
