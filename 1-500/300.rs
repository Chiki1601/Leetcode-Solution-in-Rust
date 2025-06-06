impl Solution {
    pub fn length_of_lis(xs: Vec<i32>) -> i32 {
        assert!(!xs.is_empty());
        let mut dp = vec![1; xs.len()];
        for (r, &rx) in xs.iter().enumerate().skip(1) {
            for (l, &lx) in xs.iter().enumerate().take(r) {
                if rx > lx {
                    dp[r] = dp[r].max(dp[l] + 1);
                }
            }
        }
        dp.into_iter().max().unwrap()
    }
}
