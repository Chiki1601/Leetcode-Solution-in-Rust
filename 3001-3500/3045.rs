use std::collections::HashMap;
impl Solution {
    pub fn count_prefix_suffix_pairs(words: Vec<String>) -> i64 {
        let mut ans = 0;
        let mut map: HashMap<&str, i64> = HashMap::new();
        
        for word in &words {
            for (k, v) in map.iter() {
                if k.len() <= word.len() &&
                   word.starts_with(k) &&
                   word.ends_with(k) {
                       ans += v;
                }
            }
            
            *map.entry(&word).or_insert(0) += 1;
        }
        
        ans
    }
}
