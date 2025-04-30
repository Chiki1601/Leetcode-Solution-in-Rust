impl Solution {
    pub fn find_numbers(nums: Vec<i32>) -> i32 {
        nums
            .into_iter()
            .filter(|x| {
                x.to_string().len() % 2 == 0
            })
            .count() as i32
    }
}
