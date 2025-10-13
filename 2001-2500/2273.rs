use std::collections::{BinaryHeap, HashSet};

impl Solution {
    pub fn remove_anagrams(words: Vec<String>) -> Vec<String> {
        let sorted_words = words
            .iter()
            .map(|word| word.chars().collect::<BinaryHeap<_>>().into_sorted_vec())
            .collect::<Vec<_>>();
        let whitelist = sorted_words
            .iter()
            .enumerate()
            .filter(|&(i, right)| sorted_words.get(i - 1).is_none_or(|left| left != right))
            .map(|(i, _)| i)
            .collect::<HashSet<_>>();
        words
            .into_iter()
            .enumerate()
            .filter(|(i, _)| whitelist.contains(i))
            .map(|(_, word)| word)
            .collect()
    }
}
