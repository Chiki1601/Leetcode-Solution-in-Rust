impl Solution 
{
    pub fn minimum_total_distance(mut rob: Vec<i32>, mut fac: Vec<Vec<i32>>) -> i64 
    {
        rob.sort(); fac.sort();
        let (n, m) = (rob.len(), fac.len());
        
        let mut dp: Vec<i64> = vec![1000000000000000;n+1];
        dp[0] = 0;
        
        for (j, f) in fac.iter().enumerate()
        {
            for _ in 0..f[1]
            {
                for (i, &r) in rob.iter().enumerate().rev()
                {
                    dp[i+1] = dp[i+1].min((r-f[0]).abs() as i64 + dp[i]);
                }
            }
        }
        
        return dp[n];
    }
}
