impl Solution {
    pub fn get_words_in_longest_subsequence(w: Vec<String>, g: Vec<i32>) -> Vec<String> {
        let mut dp = vec![Vec::<String>::new(); w.len() + 1];
        for i in 0..dp.len() { for j in 0..i { if 
            i == w.len() || g[i] != g[j] && w[i].len() == w[j].len() &&
            w[i].bytes().zip(w[j].bytes()).filter(|(a, b)| a != b).count() < 2 {
            if dp[j].len() + 1 > dp[i].len() { let mut s = dp[j].clone(); s.push(w[j].clone()); dp[i] = s }}}}
        dp[w.len()].clone()
    }
}
