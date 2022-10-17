use std::collections::HashSet;
impl Solution {
    pub fn check_if_pangram(sentence: String) -> bool {
        let mut set = HashSet::new();
        
        for s in sentence.chars() {
            set.insert(s);
        }
        
        set.len() == 26 
    }
}
