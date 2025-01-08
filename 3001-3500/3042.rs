impl Solution {
    pub fn is_prefix_and_suffix(s1: &str, s2: &str) -> bool {
        s2.starts_with(s1) && s2.ends_with(s1)
    }

    pub fn count_prefix_suffix_pairs(words: Vec<String>) -> i32 {
        let mut res = 0;
        for i in 0..words.len() {
            for j in i + 1..words.len() {
                if Solution::is_prefix_and_suffix(&words[i], &words[j]) {
                    res += 1
                }
            }
        }
        res
    }
}
