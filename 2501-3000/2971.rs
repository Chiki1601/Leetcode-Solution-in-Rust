impl Solution {
    pub fn largest_perimeter(mut nums: Vec<i32>) -> i64 {
        let mut sum = 0i64;
        let mut ans = -1;
        nums.sort_unstable();

        for i in 0..nums.len() {
            let num = nums[i] as i64;
            if num < sum {
                ans = sum + num
            }
            sum += num
        }

        ans
    }
}
