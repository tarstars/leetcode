use crate::Solution;

use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Debug, Default)]
struct AhoNode {
    term_id: Option<usize>,
    suffix: usize,
    children: HashMap<char, usize>,
    go: HashMap<char, usize>,
}

impl AhoNode {
    fn create_aho(words: &[String]) -> Vec<AhoNode> {
        let mut automaton: Vec<AhoNode> = Vec::new();

        automaton.push(AhoNode::default());

        for (index_word, current_word) in words.iter().enumerate() {
            let mut tree_pos = 0;
            for c in current_word.chars() {
                if !automaton[tree_pos].children.contains_key(&c) {
                    let new_index = automaton.len();
                    automaton.push(AhoNode::default());
                    automaton[tree_pos].children.insert(c, new_index);
                }
                tree_pos = *automaton[tree_pos].children.get(&c).unwrap();
            }
            automaton[tree_pos].term_id = Some(index_word);
        }

        let mut q: VecDeque<usize> = VecDeque::new();
        q.push_back(0);

        while let Some(current_pos) = q.pop_front() {
            let children_copy = automaton[current_pos].children.clone();
            let suff = automaton[current_pos].suffix;
            let go_inherited = automaton[suff].go.clone();
            automaton[current_pos].go.extend(go_inherited);
            automaton[current_pos].go.extend(children_copy.clone());

            for (c, ind) in children_copy.iter() {
                if current_pos != 0 {
                    automaton[*ind].suffix = *automaton[suff].go.get(c).unwrap_or(&0);
                }
                q.push_back(*ind);
            }
        }

        automaton
    }
}

impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        if words.is_empty() {
            return vec![];
        }

        let word_length = words[0].len();
        let total_words = words.len();
        let total_length = word_length * total_words;

        if word_length == 0
            || words.iter().any(|word| word.len() != word_length)
            || s.len() < total_length
        {
            return vec![];
        }

        let mut word_count: HashMap<String, usize> = HashMap::new();

        for current_word in words {
            *word_count.entry(current_word).or_insert(0) += 1;
        }

        let uniq_words: Vec<String> = word_count.keys().cloned().collect();
        let automaton = AhoNode::create_aho(&uniq_words);

        let id_count: Vec<i32> = uniq_words
            .iter()
            .map(|word| word_count[word] as i32)
            .collect();

        let mut a_match = vec![None; s.len() + 1 - word_length];

        let mut pos = 0;
        for (c_ind, c_c) in s.chars().enumerate() {
            pos = *automaton[pos].go.get(&c_c).unwrap_or(&0);
            if let Some(match_id) = automaton[pos].term_id {
                a_match[c_ind + 1 - word_length] = Some(match_id);
            }
        }

        let mut positions = Vec::new();

        for offset in 0..word_length {
            let mut non_zero = id_count.len();
            let mut freq = id_count.clone();

            let mut p = offset;
            let mut window: VecDeque<usize> = VecDeque::new();

            while p < a_match.len() {
                match a_match[p] {
                    Some(match_id) => {
                        window.push_back(match_id);
                        freq[match_id] -= 1;
                        if freq[match_id] == 0 {
                            non_zero -= 1;
                        }

                        while freq[match_id] < 0 {
                            let free_c = window.pop_front().unwrap();
                            if freq[free_c] == 0 {
                                non_zero += 1;
                            }
                            freq[free_c] += 1;
                        }

                        if non_zero == 0 {
                            positions.push((p + word_length - total_length) as i32);
                        }
                    }
                    None => {
                        window.clear();
                        freq = id_count.clone();
                        non_zero = id_count.len();
                    }
                }
                p += word_length;
            }
        }

        positions
    }
}
