impl Solution {
    pub fn max_operations(nums: Vec<i32>) -> i32 {
        let mut ans = 1;
        let n = nums.len();

        for (l, r) in [(0, 1), (0, n - 1), (n - 2, n - 1)] {
            let mut memo = vec![vec![-1; n]; n];
            ans = ans.max(dp(0, n - 1, nums[l] + nums[r], &nums, &mut memo));
        }
        
        ans
    }
}

fn dp(l: usize, r: usize, target: i32, nums: &Vec<i32>, memo: &mut Vec<Vec<i32>>) -> i32 {
    if r > nums.len() || l >= r { return 0 };
    if memo[l][r] != -1 { return memo[l][r] };
    
    let mut ans = 0;
    
    if nums[l] + nums[l + 1] == target {
        ans = ans.max(1 + dp(l + 2, r, target, nums, memo));
    } if nums[r] + nums[r - 1] == target {
        ans = ans.max(1 + dp(l, r - 2, target, nums, memo));
    } if nums[l] + nums[r] == target {
        ans = ans.max(1 + dp(l + 1, r - 1, target, nums, memo));
    }
    
    memo[l][r] = ans;
    ans
}
