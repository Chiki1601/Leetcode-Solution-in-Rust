use std::collections::HashMap;

impl Solution {
    pub fn count_good(nums: Vec<i32>, k: i32) -> i64 {
        let mut count = HashMap::new();
        let mut pairs = 0i64;
        let mut left = 0;
        let mut res = 0;

        for right in 0..nums.len() {
            let c = count.entry(nums[right]).or_insert(0);
            pairs += *c as i64;
            *c += 1;

            while pairs >= k as i64 {
                res += (nums.len() - right) as i64;
                let c = count.get_mut(&nums[left]).unwrap();
                *c -= 1;
                pairs -= *c as i64;
                left += 1;
            }
        }
        res
    }
}
