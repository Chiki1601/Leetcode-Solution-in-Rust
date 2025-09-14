use std::collections::{HashMap, HashSet};

fn is_vowel(c: char) -> bool {
    matches!(c, 'a'|'e'|'i'|'o'|'u')
}

fn mask_vowels(s: &str) -> String {
    s.chars().map(|c| if is_vowel(c) { 'a' } else { c }).collect()
}

impl Solution {
    pub fn spellchecker(wordlist: Vec<String>, queries: Vec<String>) -> Vec<String> {
        let mut exact: HashSet<String> = HashSet::new();
        let mut lower_map: HashMap<String, String> = HashMap::new();
        let mut vowel_map: HashMap<String, String> = HashMap::new();

        for w in &wordlist {
            exact.insert(w.clone());
        }
        for w in &wordlist {
            let wl = w.to_lowercase();
            lower_map.entry(wl.clone()).or_insert(w.clone());
            let masked = mask_vowels(&wl);
            vowel_map.entry(masked).or_insert(w.clone());
        }

        let mut ans = Vec::with_capacity(queries.len());
        for q in queries {
            if exact.contains(&q) {
                ans.push(q);
                continue;
            }
            let ql = q.to_lowercase();
            if let Some(orig) = lower_map.get(&ql) {
                ans.push(orig.clone());
                continue;
            }
            let qmask = mask_vowels(&ql);
            if let Some(orig) = vowel_map.get(&qmask) {
                ans.push(orig.clone());
            } else {
                ans.push(String::new());
            }
        }
        ans
    }
}
