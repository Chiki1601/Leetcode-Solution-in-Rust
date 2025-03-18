impl Solution {
    pub fn longest_nice_subarray(nums: Vec<i32>) -> i32 {
        let mut max_length = 1;
        let mut left = 0;
        let mut used_bits = 0;

        for right in 0..nums.len() {
            while (used_bits & nums[right]) != 0 {
                used_bits ^= nums[left];
                left += 1;
            }

            used_bits |= nums[right];
            max_length = max_length.max((right - left + 1) as i32);
        }

        max_length
    }
}
