impl Solution {
    pub fn count_pairs(nums: Vec<i32>, k: i32) -> i32 {
        let mut cnt = 0;
        for i in 0..nums.len() - 1 {
            for j in i + 1..nums.len() {
                if nums[i] == nums[j] && i * j % k as usize == 0 {
                    cnt += 1;
                }
            }
        }
        cnt
    }
}
