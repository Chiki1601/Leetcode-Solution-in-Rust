impl Solution {
    pub fn min_bitwise_array(mut nums: Vec<i32>) -> Vec<i32> {
        for num in &mut nums {
            match num.trailing_ones() {
                0 => *num = -1,
                to => *num ^= 1 << to - 1,
            }
        }
        nums
    }
}
