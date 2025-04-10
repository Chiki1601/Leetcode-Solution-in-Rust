impl Solution {
    pub fn number_of_powerful_int(start: i64, finish: i64, limit: i32, s: String) -> i64 {
        // Check that every digit in s is at most `limit`.
        if s.bytes().any(|ch| ch > b'0' + limit as u8) {
            return 0;
        }

        // Determine the power of 10 based on the length of s.
        let pow_s = 10i64.pow(s.len() as u32);
        let suffix = s.parse::<i64>().unwrap();

        // If finish is less than suffix, there are no valid numbers.
        if finish < suffix {
            return 0;
        }

        // Ensure the starting value is not less than suffix.
        let start = if start < suffix { suffix } else { start };

        // Compute adjusted quotient values after dividing by pow_s.
        // The adjustment is done by checking whether the remainder (start_value % pow_s) is
        // strictly greater than (or greater than or equal to) suffix.
        let start_div = start / pow_s + if start % pow_s > suffix { 1 } else { 0 };
        let finish_div = finish / pow_s + if finish % pow_s >= suffix { 1 } else { 0 };

        // Helper function to convert the given number into a base (limit + 1) representation.
        // It also performs a correction: if any digit exceeds the limit, it "carries" over.
        fn base_limit(mut num: i64, limit: i64) -> i64 {
            let mut result = 0;
            let mut base_mul = 1;

            while num > 0 {
                let cur = num % 10;
                num /= 10;

                if cur > limit {
                    // If the current digit exceeds the allowed limit, increment the remaining number,
                    // and reset the result (since the previous calculation needs re-adjustment).
                    num += 1;
                    result = 0;
                } else {
                    result += cur * base_mul;
                }

                base_mul *= limit + 1;
            }

            result
        }

        // The final result is the difference between the converted finish_div and start_div values.
        base_limit(finish_div, limit as i64) - base_limit(start_div, limit as i64)
    }
}
