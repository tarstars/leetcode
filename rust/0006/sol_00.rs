use crate::Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let num_rows = num_rows as usize;
        if num_rows == 1 {
            return s;
        }
        let n = s.len();
        let mut result = String::with_capacity(n);
        let buf = s.as_bytes();

        for sp in 0..num_rows {
            if sp == 0 || sp == (num_rows - 1) {
                let mut p = sp;
                while p < n {
                    result.push(buf[p] as char);
                    p += 2*num_rows - 2;
                }
            } else {
                let mut p = sp;
                while p < n {
                    result.push(buf[p] as char);
                    p += 2 * num_rows - 2 - 2*sp;
                    if p >= n {
                        break;
                    }
                    result.push(buf[p] as char);
                    p += 2* sp;
                }
            }
        }

        result
    }
}
