impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        match nums.iter().sum::<i32>() {
            x if x % 2 == 1 => false,
            x => nums.iter().fold((vec![false; (x / 2 + 1) as usize], vec![false; (x / 2 + 1) as usize]), |(mut dp1, mut dp2), &num| {
                dp1[0] = true;
                (1..=x / 2).for_each(|i| {
                    dp2[i as usize] = dp1[i as usize] || (i >= num && dp1[(i - num) as usize]);
                });
                (dp2, dp1)
            }).1[(x / 2) as usize]
        }
    }
}
