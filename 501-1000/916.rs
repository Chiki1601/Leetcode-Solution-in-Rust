impl Solution {
    pub fn word_subsets(a: Vec<String>, b: Vec<String>) -> Vec<String> {
        let char_counts = |s: &String| -> [usize; 26] {
            s.as_bytes().iter().fold([0; 26], |mut acc, &u| {
                acc[(u - b'a') as usize] += 1;
                acc
            })
        };
        let target = b.iter().map(char_counts).fold(vec![0; 26], |acc, x| {
            acc.iter()
                .enumerate()
                .map(|(i, &count)| x[i].max(count))
                .collect()
        });
        a.into_iter()
            .filter(|s| {
                char_counts(s)
                    .iter()
                    .enumerate()
                    .all(|(i, &count)| count >= target[i])
            })
            .collect()
    }
}
