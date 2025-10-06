impl Solution {
    pub fn longest_subsequence(nums: Vec<i32>) -> i32 {
        let mut xor_sum = 0;
        let mut all_zero = true;

        for &num in nums.iter() {
            xor_sum ^= num;
            if num != 0 {
                all_zero = false;
            }
        }

        if all_zero {
            return 0;
        }

        if xor_sum != 0 {
            nums.len() as i32
        } else {
            (nums.len() as i32) - 1
        }
    }
}
