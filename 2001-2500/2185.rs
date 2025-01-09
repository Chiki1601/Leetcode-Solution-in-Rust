impl Solution {
    pub fn prefix_count(words: Vec<String>, pref: String) -> i32 {
        words.iter().fold(0, |a, x| { a + if x.starts_with(&pref) { 1 } else { 0 } })
    }
}
