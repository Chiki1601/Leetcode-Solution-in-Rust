impl Solution {
    pub fn number_of_cuts(n: i32) -> i32 {
        match n % 2 {
            1 if n == 1 => 0,
            1 => n,
            _ => n / 2,
        }
    }
}
