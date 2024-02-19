impl Solution {
    pub fn max_operations(nums: Vec<i32>) -> i32 {
        nums.chunks_exact(2)
            .map(|pair| pair[0] + pair[1])
            .collect::<Vec<_>>()
            .windows(2)
            .take_while(|sum| sum[0] == sum[1])
            .count() as i32
            + 1
    }
}
