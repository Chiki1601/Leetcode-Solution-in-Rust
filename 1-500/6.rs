impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }
        let s: Vec<_> = s.chars().collect();
        let mut array = vec![String::new(); num_rows as usize];
        let (mut is_neg, mut row) = (false, 0);
        for idx in 0..s.len() {
            array[row].push(s[idx]);
            if row == 0 {
                is_neg = false;
            }
            if row == (num_rows - 1) as usize {
                is_neg = true;
            }
            match is_neg {
                true  => row -= 1,
                false => row += 1,
            }
        }
        array.join("").to_string()
    }
}
