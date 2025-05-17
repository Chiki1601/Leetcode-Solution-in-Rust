impl Solution {
 
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let (mut cnt, mut j) = ([0, 0, 0], 0);
        for &n in &*nums { cnt[n as usize] += 1 }
        for i in 0..cnt.len() {
            nums[j..j + cnt[i]].fill(i as _);
            j += cnt[i]
        }
    }
}
