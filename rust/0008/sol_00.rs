use crate::Solution;

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let s = s.as_bytes();
        let mut sign = 1;
        let n = s.len();
        let mut val: i64 = 0;
        let mut met_digit = false;
        let mut met_sign = false;

        for p in 0..n {
            if s[p] == b' ' || s[p] == b'\n' || s[p] == b'\t' {
                if met_digit || met_sign {
                    break
                }
                continue;
            }     
            if s[p] == b'-' {
                if met_digit || met_sign{
                    break
                }
                sign = -1;
                met_sign = true;
                continue;
            }
            if s[p] == b'+' {
                if met_sign || met_digit {
                    break;
                }
                met_sign = true;
                continue;
            }
            if s[p].is_ascii_digit() {
                val = val * 10 + (s[p] - b'0') as i64;
                met_digit = true;
            }
            if sign == 1 && val >= 2147483647 {
                return 2147483647
            }
            if sign == -1 && val >= 2147483648 {
                return -2147483648;
            }
            if s[p].is_ascii_alphabetic() || s[p] == b'.' {
                break;
            }
        }

        sign * val as i32
    }
}
