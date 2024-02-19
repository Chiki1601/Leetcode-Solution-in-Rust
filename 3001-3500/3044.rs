use std::collections::HashMap;

impl Solution {

  fn is_prime(n: i32) -> bool {
    if n <= 2 {
      return false;
    }

    for i in 2 ..= (n as f64).sqrt() as i32 {
      if n % i == 0 {
        return false;
      }
    }

    return true;
  }

  pub fn most_frequent_prime(M: Vec<Vec<i32>>) -> i32 {
    let mut nums: HashMap<i32, i32> = HashMap::new();
    let (Y, X) = (M.len(), M[0].len());
    let dirs = [-1, 0, 1, -1, 1, 1, 0, -1, -1];

    for y in 0 .. Y {
      for x in 0 .. X {
        for i in 0 .. 8 {
          let (mut v, mut y_, mut x_) = (M[y][x], y, x);
          while ((y_ as i32 + dirs[i]) as usize) < Y && ((x_ as i32 + dirs[i + 1]) as usize) < X {
            y_ = (y_ as i32 + dirs[i]) as usize;
            x_ = (x_ as i32 + dirs[i + 1]) as usize;
            v = v * 10 + M[y_][x_];
            if Self::is_prime(v) {
              *nums.entry(v).or_default() += 1;
            }
          }
        }
      }
    }

    if nums.is_empty() {
      return -1;
    }

    let (mut cnt, mut res) = (0, 0);
    for c in nums.values() {
      cnt = cnt.max(*c);
    }

    for (v, c) in nums {
      if c == cnt {
        res = res.max(v);
      }
    }

    return res;
  }
}
