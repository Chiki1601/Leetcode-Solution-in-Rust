impl Solution {
pub fn kth_character(k: i64, operations: Vec<i32>) -> char {
    let k_bits = format!("{:01$b}", k-1, operations.len());
    let plus: u32 = k_bits.chars().zip(operations.iter().rev()).map(
        |(x, &y)| (x == '1' && y == 1) as u32
    ).sum();

    ('a' as u8 + (plus % 26) as u8) as char
}
}
