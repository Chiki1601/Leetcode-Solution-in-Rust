impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut l = [0;101];
        for num in nums{
            if num > k{
                l[num as usize] = 1;
            } else if num < k{
                return -1;
            }
        }
        l.iter().sum()
    }
}
