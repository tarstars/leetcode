use crate::Solution;

struct RowStat {
    string_cluster: Vec<usize>,
    free_space: usize,
    last: bool,
}

impl RowStat {
    fn justify_left(self: &Self, words: &Vec<String>, width: usize) -> String {
        let picked: Vec<&str> = self
            .string_cluster
            .iter()
            .map(|ind| words[*ind].as_str())
            .collect();
        let join_words = picked.join(" ");

        format!("{join_words:<width$}")
    }

    fn justify_center(self: &Self, words: &Vec<String>, width: usize) -> String {
        let words: Vec<&String> = self.string_cluster.iter().map(|&q| &words[q]).collect();
        if words.len() == 1 {
            let word = words[0].clone();
            return format!("{word:<width$}");
        }

        let mut to_collect: Vec<String> = Vec::new();

        to_collect.push(words[0].clone());
        let n = words.len() - 1;
        let base_num = self.free_space / n;
        let num_incr = self.free_space % n;

        for p in 0..n {
            let mut v = base_num + 1;
            if p < num_incr {
                v += 1;
            }
            to_collect.push(" ".repeat(v));
            to_collect.push(words[p + 1].clone());
        }

        to_collect.join("")
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
                    string_cluster.push(RowStat {
                        string_cluster: row,
                        free_space: row_space,
                        last: false,
                    });
                    row_space = max_width - word.len();
                    row = vec![ind];
                }
            }
        }
        string_cluster.push(RowStat {
            string_cluster: row,
            free_space: row_space,
            last: true,
        });

        string_cluster
            .iter()
            .map(|row| {
                if row.last || row.string_cluster.len() == 1 {
                    row.justify_left(&words, max_width)
                } else {
                    row.justify_center(&words, max_width)
                }
            })
            .collect()
    }
}
