impl Solution {
    pub fn string_matching(mut words: Vec<String>) -> Vec<String> {
        let (n, mut res) = (words.len(), vec![]);
        words.sort_by_key(|w| w.len());

        for i in 0..n {
            for j in (i+1)..n {
                if words[j].contains(&words[i]) {
                    res.push(words[i].clone());
                    break;
                }
            }
        }

        res
    }
}
