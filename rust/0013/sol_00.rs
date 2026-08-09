use std::collections::HashMap;

use crate::Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let s = s.as_bytes();
        let mut v = 0;

        let r2i = [
            (b'I',	1),
            (b'V',	5),
            (b'X',	10),
            (b'L',	50),
            (b'C',	100),
            (b'D',	500),
            (b'M',	1000),
        ];

        let mut hash: HashMap<u8, i32> = HashMap::new();
        for ri in r2i {
            hash.insert(ri.0, ri.1);
        }

        let n = s.len();

        for p in 0..n {
            if p + 1 < n && hash.get(&s[p]) < hash.get(&s[p+1]) {
                v -= hash.get(&s[p]).unwrap();
            } else {
                v += hash.get(&s[p]).unwrap();
            }
        }

        v
    }
}
