impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        match nums.len() {
            1 => nums[0],
            _ => {
                let l = nums.len();
                let mut max_money = Vec::with_capacity(l);
                for (i, &m) in nums.iter().enumerate() {
                    let prev_max = match i {
                        0 | 1 => 0,
                        2 => max_money[0],
                        _ => max_money[i - 2].max(max_money[i - 3]),
                    };
                    max_money.push(m + prev_max)
                }
                max_money[l - 1].max(max_money[l - 2])
            }
        }
    }
}
