impl Solution {
    pub fn has_increasing_subarrays(nums: Vec<i32>, k: i32) -> bool {
        if k == 1 {
            return true;
        }
        let k = k as usize;
        let n = nums.len();
        for i in 1..n - 2 * k  + 2 {
            let mut valid = true;
            for j in 0..k - 1 {
                if nums[i + j ] <= nums[i + j  - 1] || nums[i + j  + k ] <= nums[i + j  + k  - 1] {
                    valid = false;
                    break;
                }
            }
            if valid {
                return true;
            }
        }
        false
    }
}
