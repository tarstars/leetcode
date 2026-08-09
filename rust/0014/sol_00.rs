use crate::Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut p: usize = 0;

        loop {
            let mut inc_pass = true;
            let mut first = true;
            let mut common_letter = b' ';
            for row in &strs {
                if p >= row.len() {
                    inc_pass = false; 
                    break;
                }
                if first {
                    first = false;
                    common_letter = row.as_bytes()[p];
                } else {
                    if common_letter != row.as_bytes()[p] {
                        inc_pass = false;
                        break;
                    }
                }
            }
            if inc_pass {
                p += 1;
            } else {
                break;
            }
        }

        strs[0][0..p].to_string()
    }
}
