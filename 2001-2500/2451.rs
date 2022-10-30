impl Solution {
    pub fn odd_string(words: Vec<String>) -> String {
        let get_diff_vec = |s: &str| -> Vec<i32> {
            s[1..]
                .bytes()
                .zip(s.bytes())
                .map(|(b, a)| b as i32 - a as i32)
                .collect()
        };

        let mut map = std::collections::HashMap::with_capacity(2);
        for word in words.into_iter() {
            let key = get_diff_vec(word.as_str());
            let (count, _) = map.entry(key).or_insert((0, word));

            *count += 1;
            if *count > 1 && map.len() > 1 {
                break;
            }
        }

        map.into_values()
            .find_map(|(count, word)| (count == 1).then(|| word))
            .unwrap()
    }
}
