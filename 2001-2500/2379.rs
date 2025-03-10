impl Solution {
    pub fn minimum_recolors(blocks: String, k: i32) -> i32 {
        let mut black_count = 0;
        let mut ans = i32::MAX;
        let k = k as usize;
        let blocks: Vec<char> = blocks.chars().collect();

        for i in 0..blocks.len() {
            if i >= k && blocks[i - k] == 'B' {
                black_count -= 1;
            }
            if blocks[i] == 'B' {
                black_count += 1;
            }
            ans = ans.min(k as i32 - black_count);
        }

        ans
    }
}
