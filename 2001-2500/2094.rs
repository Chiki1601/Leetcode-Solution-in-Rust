impl Solution {
    pub fn find_even_numbers(digits: Vec<i32>) -> Vec<i32> {
        let freqs = digits.into_iter().fold([0; 10], |mut f, d| {
            f[d as usize] += 1;
            f
        });
        (100..999)
            .step_by(2)
            .filter(|num| {
                let mut num = *num;
                let mut freq = freqs;
                let mut digit;
                while num > 0 {
                    digit = (num % 10) as usize;
                    if freq[digit] == 0 {
                        return false;
                    }
                    freq[digit] -= 1;
                    num /= 10;
                }
                true
            })
            .collect()
    }
}
