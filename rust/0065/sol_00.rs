use crate::Solution;

fn is_integer(s: &str) -> Option<()> {
    let mut it = s.chars().into_iter().peekable();
    it.next_if(|&c| c == '+' || c == '-');

    it.next_if(|&c| c.is_ascii_digit())?;
    while it.next_if(|&c| c.is_ascii_digit()).is_some() {}

    it.peek().is_none().then_some(())
}

fn is_float(s: &str) -> Option<()> {
    let mut it = s.chars().into_iter().peekable();
    let mut has_digit = false;
    it.next_if(|&c| c == '+' || c == '-');
    while it.next_if(|c| c.is_ascii_digit()).is_some() {
        has_digit = true;
    }
    it.next_if(|&c| c == '.')?;
    while it.next_if(|c| c.is_ascii_digit()).is_some() {
        has_digit = true;
    }
    (it.peek().is_none() && has_digit).then_some(())
}

impl Solution {
    pub fn is_number(s: String) -> bool {
        match s.split_once(['e', 'E']) {
            Some((left, right)) => {
                (is_float(left).is_some() || is_integer(left).is_some())
                    && is_integer(right).is_some()
            }
            _ => is_integer(&s).is_some() || is_float(&s).is_some(),
        }
    }
}
