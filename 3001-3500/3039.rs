impl Solution {
    pub fn last_non_empty_string(s: String) -> String {
        let mut map = [0i32; 128];
        
        for ch in s.chars() {
            map[ch as usize] += 1;
        }
        
        let max_occurrences = map.iter().max().unwrap() - 1;
        
        // Use iterators instead of loops
        let ans: String = s.chars()
            .rev()
            // Use match instead of if else
            .filter(|&ch| match map[ch as usize] {
                n if n > max_occurrences => {
                    map[ch as usize] -= 1;
                    true
                }
                _ => false
            })
            .collect();
        
        ans.chars().rev().collect()
    }
}
