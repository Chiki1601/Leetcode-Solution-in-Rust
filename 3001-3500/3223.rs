impl Solution {
    pub fn minimum_length(s: String) -> i32 {
        let mut counts = vec![0; 26];
        let mut res = 0;

        for b in s.as_bytes() {
            let k = (b - b'a') as usize;
            counts[k] += 1;
            res += 1;
            if counts[k] == 3 {
                counts[k] -= 2;
                res -= 2;
            }
        }

        res
    }
}
