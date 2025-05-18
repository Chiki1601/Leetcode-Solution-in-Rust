use std::collections::HashMap;

impl Solution {
    pub fn color_the_grid(m: i32, n: i32) -> i32 {
        let m = m as usize;
        let n = n as usize;
        Self::dfs_ct(m, n, 0, &mut HashMap::new(), &mut HashMap::new()) as i32
    }

    fn dfs_ct(
        m: usize,
        i: usize,
        prev: u16,
        next_cols_cache: &mut HashMap<u16, Vec<u16>>,
        ct_cache: &mut HashMap<(u16, usize), usize>,
    ) -> usize {
        if i == 0 { return 1; }
        if let Some(cached) = ct_cache.get(&(prev, i)) {
            return *cached;
        }

        let next_cols = match next_cols_cache.get(&prev) {
            Some(a) => a.clone(),
            None => {
                let mut res = vec![];
                Self::calc_next_cols(m, prev, 0, 0, &mut res);
                next_cols_cache.insert(prev, res.clone());
                res
            }
        };
        let mut ans = 0;
        for col in next_cols {
            ans += Self::dfs_ct(m, i - 1, col, next_cols_cache, ct_cache);
            ans %= 1_000_000_007;
        }
        ct_cache.insert((prev, i), ans);
        return ans;
    }

    fn calc_next_cols(m: usize, prev: u16, i: usize, acc: u16, res: &mut Vec<u16>) {
        if i == m {
            res.push(acc);
            return;
        }

        let mut r = true;
        let mut g = true;
        let mut b = true;

        let v = (prev >> (i * 2)) & 0b11;
        if v == 1 { r = false; }
        if v == 2 { g = false; }
        if v == 3 { b = false; }
        if i > 0 {
            let v = acc >> ((i - 1) * 2) & 0b11;
            if v == 1 { r = false; }
            if v == 2 { g = false; }
            if v == 3 { b = false; }
        }
        if r {
            Self::calc_next_cols(m, prev, i + 1, acc | 1 << i * 2, res);
        }
        if g {
            Self::calc_next_cols(m, prev, i + 1, acc | 2 << i * 2, res);
        }
        if b {
            Self::calc_next_cols(m, prev, i + 1, acc | 3 << i * 2, res);
        }
    }
}
