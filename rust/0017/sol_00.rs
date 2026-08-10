use crate::Solution;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return vec![];
        }
        
        let dec = vec![
            "", "", "abc",
            "def", "ghi", "jkl",
            "mno", "pqrs", "tuv",
            "wxyz"];
        
        let mut ret :Vec<String> = vec![String::new()];

        for current_digit in digits.bytes() {
            if current_digit != b'1' {
                let prev = ret;
                ret = vec![];
                for digit in dec[(current_digit - b'0') as usize].chars() {
                    for cluster in &prev {
                        let mut cluster_upd = cluster.clone();
                        cluster_upd.push(digit);
                        ret.push(cluster_upd);
                    }
                }
            }
        }

        ret
    }
}
