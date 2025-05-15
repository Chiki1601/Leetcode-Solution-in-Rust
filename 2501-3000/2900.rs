impl Solution {
  fn get_longest_subsequence(words: Vec<String>, groups: Vec<i32>) -> Vec<String> {
    fn build(words: &Vec<String>, groups: &Vec<i32>, start: i32) -> Vec<String> {
        let mut res = Vec::new();
        let mut expect = start;
        for i in 0..words.len() {
            if groups[i] == expect {
                res.push(words[i].clone());
                expect ^= 1;
            }
        }
        res
    }
    let res1 = build(&words, &groups, 0);
    let res2 = build(&words, &groups, 1);
    if res1.len() >= res2.len() {
        res1
    } else {
        res2
    }
}
}
