impl Solution {
    pub fn clear_digits(s: String) -> String {
        let mut res = vec![];
        for c in s.into_bytes() {
            if c.is_ascii_digit() {
                res.pop();
            } else {
                res.push(c);
            }
        }
        String::from_utf8(res).unwrap()
    }
}
