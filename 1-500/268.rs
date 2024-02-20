impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let mut s1 = nums.len();
        let mut s2 = 0;
        for (i, v) in nums.iter().enumerate() {
            s1 += i;
            s2 += v;
        }
        s1 as i32 - s2
    }
}
