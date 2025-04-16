use std::collections::HashMap;

impl Solution {
    pub fn count_good(nums: Vec<i32>, k: i64) -> i64 {
        let mut l = 0;
        let mut count = 0;
        let mut ans = 0;
        let mut mp = HashMap::new();
        for r in 0..nums.len() {
            let entry = mp.entry(nums[r]).or_insert(0);
            count += *entry;
            *entry += 1;
            while l < nums.len() && (count - (mp[&nums[l]] - 1)) as i64 >= k {
                mp.insert(nums[l], mp[&nums[l]] - 1);
                count -= mp[&nums[l]];
                l += 1;
            }
            if count as i64 >= k {
                ans += (l + 1) as i64;
            }
        }
        ans
    }
}
