impl Solution {
     
        pub fn check_inclusion(smaller: String, bigger: String) -> bool {
        use std::collections::HashMap;

        if bigger.len() < smaller.len() {
            return false;
        }

        let mut frequency_smaller_word = HashMap::new();

        for c in smaller.chars() {
            let mut count = frequency_smaller_word.entry(c).or_insert(0);
            *count += 1;
        }

        let mut frequency = HashMap::new();
        let mut window_start = 0_usize;
        let chars = bigger.chars().collect::<Vec<_>>();

        for idx in window_start..smaller.len() - 1 {
            let c = chars[idx];
            let mut count = frequency.entry(c).or_insert(0);
            *count += 1;
        }

        for idx in smaller.len() - 1..bigger.len() {
            let mut count = frequency.entry(chars[idx]).or_insert(0);
            *count += 1;

            if frequency_smaller_word == frequency {
                return true;
            }

            let to_remove = chars[window_start];
            if let Some(count) = frequency.remove(&to_remove) {
                if count > 1 {
                    frequency.insert(to_remove, count - 1);
                }
            }
            window_start += 1;
        }

        false
    }
    
}
