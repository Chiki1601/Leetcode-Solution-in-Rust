impl Solution {
    pub fn sum_zero(n: i32) -> Vec<i32> {
        std::iter::once(-n * (n - 1) / 2).chain(1..n).collect()
    }
}
