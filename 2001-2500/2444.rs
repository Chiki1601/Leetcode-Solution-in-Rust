impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, min_k: i32, max_k: i32) -> i64 {
        let mut res = 0i64;
        let (mut bad, mut left, mut right) = (-1, -1, -1);
        for (i, &v) in nums.iter().enumerate() {
            if v < min_k || v > max_k { bad = i as i32; }
            if v == min_k { left = i as i32; }
            if v == max_k { right = i as i32; }
            res += std::cmp::max(0, std::cmp::min(left, right) - bad) as i64;
        }
        res
    }
}
