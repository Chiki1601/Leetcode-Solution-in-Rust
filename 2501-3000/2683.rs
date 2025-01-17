impl Solution {
    pub fn does_valid_array_exist(derived: Vec<i32>) -> bool {
        return derived.into_iter().fold(1, |acc, e| acc ^ e) != 0;
    }
}
