impl Solution {
 
    pub fn max_absolute_sum(nums: Vec<i32>) -> i32 {
        let (mut a, mut b, mut r) = (0, 0, 0);
        for x in nums {
            a = x.min(a + x); b = x.max(b + x); r = b.max(-a).max(r)
        }; r
    }
}
