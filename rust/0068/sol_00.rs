use std::cmp::max;

use crate::Solution;

struct RowStat {
    string_cluster: Vec<usize>,
    free_space: usize,
    last: bool,
}

impl RowStat {
    fn justify_left(self: &Self, words: &Vec<String>, width: usize) -> String {
        let picked: Vec<&str> = self.string_cluster
            .iter()
            .map(|ind| words[*ind].as_str())
            .collect();
        let join_words = picked.join(" ");

        format!("{join_words:<width$}")
    }

    fn justify_center(self: &Self, words: &Vec<String>) -> String {
        "".to_string()
    }
}

impl Solution {
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let max_width: usize = max_width as usize;
        let mut string_cluster: Vec<RowStat> = Vec::new();

        let mut row: Vec<usize> = Vec::new();
        let mut row_space: usize = max_width as usize;

        for (ind, word) in words.iter().enumerate() {
            if ind == 0 {
                row_space -= word.len();
                row.push(ind);
            } else {
                if row_space >= word.len() + 1 {
                    row_space -= word.len() + 1;
                    row.push(ind);
                } else {
                    string_cluster.push(RowStat{string_cluster:row, free_space:row_space, last: false});
                    row_space = max_width - word.len();
                    row = vec![ind];
                }
            }
        } 
        string_cluster.push(RowStat{string_cluster:row, free_space:row_space, last: true});

        string_cluster
            .iter()
            .map(|row| if row.last || row.string_cluster.len() == 1 
                                    {
                                        row.justify_left(&words, max_width)
                                    } else {
                                        row.justify_center(&words)  
                                    }
                                    )
            .collect()
    }
}
