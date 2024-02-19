use std::collections::HashSet;

impl Solution {
  pub fn longest_common_prefix(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
    let mut data = HashSet::new();
    for mut v in arr1 {
      while v != 0 {
        data.insert(v);
        v /= 10;
      }
    }

    let mut max_v = -1;
    for mut v in arr2 {
      while v != 0 {
        if data.contains(&v) {
          max_v = max_v.max(v);
          break;
        }
        v /= 10;
      }
    }

    if max_v == -1 {
      return 0;
    }

    let mut res = 0;
    while max_v != 0 {
      max_v /= 10;
      res += 1;
    }

    return res;
  }
}
