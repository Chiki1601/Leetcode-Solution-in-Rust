impl Solution {
    pub fn number_of_substrings(s: String) -> i32 {
        let mut ans = 0;
        let mut pos = 0;
        let mut count = [0; 3];
        let v = s.as_bytes();
        for &ch in v {
            count[(ch - b'a') as usize] += 1;
            while count[0] > 0 && count[1] > 0 && count[2] > 0 {
                count[(v[pos] - b'a') as usize] -= 1;
                pos += 1;
            }
            ans += pos;
        }
        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_of_substrings() {
        assert_eq!(Solution::number_of_substrings("abcabc".to_string()), 10)
    }

    #[test]
    fn test_number_of_substrings_02() {
        assert_eq!(Solution::number_of_substrings("aaacb".to_string()), 3)
    }

    #[test]
    fn test_number_of_substrings_03() {
        assert_eq!(Solution::number_of_substrings("abc".to_string()), 1)
    }
}
