use std::collections::HashMap;

impl Solution {
    pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
        let hash: HashMap<char, usize> = order.chars().enumerate()
            .map(|(i, ch)| (ch, i))
            .collect();
        words.windows(2)
            .all(|s| s[0].chars().map(|ch| hash.get(&ch).unwrap())
                .le(s[1].chars().map(|ch| hash.get(&ch).unwrap())))
    }
}
