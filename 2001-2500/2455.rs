impl Solution {
    pub fn average_value(nums: Vec<i32>) -> i32 {
        let div = nums.into_iter().filter(|n| *n % 2 == 0 && *n % 3 == 0).collect::<Vec<_>>();
        let n = div.len() as i32;
        if n == 0 { 0 } else { div.into_iter().sum::<i32>() / n }
    }
}
