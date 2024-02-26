use std::collections::HashMap;

impl Solution {
  pub fn is_possible_to_split(nums: Vec<i32>) -> bool {
    let mut cnt = HashMap::new();
    for v in nums {
      *cnt.entry(v).or_insert(0) += 1;
    }

    for v in cnt.values() {
      if v > &2 {
        return false;
      }
    }

    return true;
  }
}
