// 55ms
impl Solution {
    pub fn rearrange_array(nums: Vec<i32>) -> Vec<i32> {
        let (mut pos, mut neg) = (0, 1);
        let mut result = vec![0; nums.len()];
        for n in nums {
            let i = if n.is_positive() { &mut pos } else { &mut neg };
            result[*i] = n;
            *i += 2;
        }
        result
    }
}
