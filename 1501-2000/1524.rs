impl Solution {
    pub fn num_of_subarrays(arr: Vec<i32>) -> i32 {
        arr.iter().fold([0, 0, 1, 0], |[n, a, e, o], &x| {
            if is_even(a + x) {
                [(n + o) % MOD, a + x, e + 1, o]
            } else {
                [(n + e) % MOD, a + x, e, o + 1]
            }
        })[0]
    }
}

const MOD: i32 = 1_000_000_007;

#[inline]
const fn is_even(x: i32) -> bool {
    x & 1 == 0
}
