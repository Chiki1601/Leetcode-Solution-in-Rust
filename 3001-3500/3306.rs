use std::collections::HashMap;

struct Solution;

impl Solution {
    fn is_vowel(c: char) -> bool {
        matches!(c, 'a' | 'e' | 'i' | 'o' | 'u')
    }

    fn at_least_k(word: &str, k: usize) -> i64 {
        let mut vowel_count = HashMap::new();
        let mut consonant_count = 0;
        let mut num_valid_substrings = 0;
        let mut start = 0;

        for (end, new_letter) in word.chars().enumerate() {
            if Self::is_vowel(new_letter) {
                *vowel_count.entry(new_letter).or_insert(0) += 1;
            } else {
                consonant_count += 1;
            }

            while vowel_count.len() == 5 && consonant_count >= k {
                num_valid_substrings += word.len() - end;
                let start_letter = word.chars().nth(start).unwrap();
                if Self::is_vowel(start_letter) {
                    if let Some(count) = vowel_count.get_mut(&start_letter) {
                        *count -= 1;
                        if *count == 0 {
                            vowel_count.remove(&start_letter);
                        }
                    }
                } else {
                    consonant_count -= 1;
                }
                start += 1;
            }
        }

        num_valid_substrings as i64
    }

    fn count_of_substrings(word: &str, k: usize) -> i64 {
        Self::at_least_k(word, k) - Self::at_least_k(word, k + 1)
    }
}

fn main() {
    println!("{}", Solution::count_of_substrings("iqeaouqi", 2));
}
