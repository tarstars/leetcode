use crate::Solution;

impl Solution {
    /// Factorial number system: O(n^2), no iteration over the k-1 permutations.
    ///
    /// Fix the leading digit and the remaining n-1 digits can still be arranged
    /// in (n-1)! ways, so the permutations fall into n blocks of (n-1)! each.
    /// With k made zero-based, `k / (n-1)!` picks the block — i.e. the index of
    /// the digit to emit — and `k % (n-1)!` is the rank within that block, which
    /// becomes the k for the next round. Repeat with one fewer candidate digit
    /// each time, and note the digit index is an index into the *remaining*
    /// candidates, not into 1..=n.
    ///
    /// n <= 9, so 8! = 40320 is the largest factorial needed and everything
    /// fits in i32.
    pub fn get_permutation(n: i32, k: i32) -> String {
        let mut digits: Vec<i32> = Vec::new();
        let mut kk = k - 1;
        for x in 1..=n {
            digits.push(kk % x);
            kk /= x;
        }

        let mut pre_perm: Vec<i32> = (1..=n).collect();
        let mut perm: Vec<i32> = Vec::new();

        while let Some(d) = digits.pop() {
            perm.push(pre_perm[d as usize]);
            pre_perm.remove(d as usize);
        }

        perm.iter().map(|x| x.to_string()).collect()
    }
}
