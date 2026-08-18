use crate::Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let haystack = haystack.as_bytes();
        let needle = needle.as_bytes();
        let sn = haystack.len();
        let pn = needle.len();

        let mut pa = vec![0; pn];

        let mut p = 0;
        for q in 1..pn {
            while p > 0 && needle[p] != needle[q] {
                p = pa[p - 1];
            }
            if needle[p] == needle[q] {
                p += 1;
                pa[q] = p;
            }
        }

        p = 0;
        for q in 0..sn {
            while p > 0 && haystack[q] != needle[p] {
                p = pa[p - 1];
            }
            if needle[p] == haystack[q] {
                p += 1;
            }
            if p == pn {
                return (q + 1 - pn) as i32;
            }
        }

        -1
    }
}
