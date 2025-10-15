impl Solution {
    pub fn max_increasing_subarrays(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let (mut up, mut pre_up, mut res) = (1, 0, 0);
        for i in 1..n {
            if nums[i] > nums[i - 1] {
                up += 1;
            } else {
                pre_up = up;
                up = 1;
            }
            let half = up >> 1;
            let min = if pre_up < up { pre_up } else { up };
            let candidate = if half > min { half } else { min };
            if candidate > res {
                res = candidate;
            }
        }
        res
    }
}
