use crate::Solution;

struct MatrixBool {
    h: usize,
    w: usize,
    dat: Vec<bool>,
}

impl MatrixBool {
    fn from_hw(h: usize, w: usize) -> Self {
        Self {h, w, dat: vec![false; h*w]}
    }

    fn get(&self, p: usize, q: usize) -> bool {
        self.dat[p*self.w + q]
    }

    fn set(&mut self, p: usize, q: usize, v: bool) {
        self.dat[p*self.w + q] = v;
    }
}

impl Solution {
    pub fn is_match(s: String, pat: String) -> bool {
        let h = s.len();
        let w = pat.len();
        let mut dp = MatrixBool::from_hw(h + 1, w + 1);

        dp.set(0, 0, true);

        for q in 1..=w {
            dp.set(0, q, dp.get(0, q - 1) && (pat.as_bytes()[q - 1] == b'*'))
        }

        for p in 1..=h {
            for q in 1..=w {
                dp.set(p, q,
                    dp.get(p - 1, q - 1) && (pat.as_bytes()[q - 1] == s.as_bytes()[p - 1] ||
                                                        pat.as_bytes()[q - 1] == b'?') ||
                    dp.get(p-1, q) && pat.as_bytes()[q - 1] == b'*' ||
                    dp.get(p, q - 1) && pat.as_bytes()[q - 1] == b'*'
                )
            }
        }

        dp.get(h, w)
    }
}
