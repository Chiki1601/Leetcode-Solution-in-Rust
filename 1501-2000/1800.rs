impl Solution {
    pub fn max_ascending_sum(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut sum = nums[0];
        nums.windows(2).for_each(|w| {
            if w[1] > w[0] {
                sum += w[1];
            } else {
                res = res.max(sum);
                sum = w[1];
            }
        });
        res.max(sum)
    }
}
