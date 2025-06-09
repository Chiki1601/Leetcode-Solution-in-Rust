impl Solution {
  
    pub fn find_kth_number(n: i32, k: i32) -> i32 {
        let (mut x, mut i, n, k) = (1, 1, n as i64, k as i64);
        while i < k {
            let (mut count, mut from, mut to) = (0, x, x);
            while from <= n {
                count += to.min(n) - from + 1;
                from *= 10; to = to * 10 + 9
            }
            if i + count <= k { i += count; x += 1 } 
            else { i += 1; x *= 10 }
        }
        x as i32
    }
}
