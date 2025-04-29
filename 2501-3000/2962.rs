impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i32) -> i64 {
        let max = *nums.iter().max().unwrap();

        let mut left = 0;
        let mut count = 0;
        let mut res : i64 = 0;

        for val in nums.iter() {
            if *val == max {
                count += 1;
            }

            while count >= k {
                if (nums[left] == max) {
                    count -= 1;
                }
                left += 1;
            }

            res += left as i64;
        }

        return res;
    }
}
