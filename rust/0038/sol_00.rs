use crate::Solution;

// use std::collections::VecDeque;

fn el_freq2seq(el: u8, mut freq: u8) -> Vec<u8>  {
    let mut seq: Vec<u8> = Vec::new();

    let mut st: Vec<u8> = Vec::new();
    while freq != 0 {
        st.push(freq % 10);
        freq = freq / 10;
    }

    while !st.is_empty() {
        seq.push(st.pop().unwrap());
    }
    seq.push(el);

    seq
}

impl Solution {
    pub fn count_and_say(n: i32) -> String {
        let mut current: Vec<u8> = vec![1];

        for _it in 0..n-1 {
            let mut next: Vec<u8> = vec![];

            let mut cur_element: Option<u8> = None;
            let mut cur_freq: Option<u8> = None;

            for v in current.iter() {
                if cur_element.is_none() || cur_element.unwrap() != *v {
                    if cur_element.is_some() {
                        next.extend(el_freq2seq(cur_element.unwrap(), cur_freq.unwrap()));
                    }
                    cur_element = Some(*v);
                    cur_freq = Some(1);
                } else {
                    cur_freq = Some(cur_freq.unwrap() + 1);
                }
            }
            next.extend(el_freq2seq(cur_element.unwrap(), cur_freq.unwrap()));
            current = next;
        }

        let mut result: String = String::new();
        for c in current {
            result.push(char::from_digit(c as u32, 10).unwrap());
        }
        result
    }
}
