impl Solution {
    fn digits_sum(mut nums: i32) -> usize {
        let mut sum = 0;
        while nums > 0 {
            sum += nums % 10;
            nums /= 10;
        }

        sum as usize
    }

    pub fn maximum_sum(nums: Vec<i32>) -> i32 {
        // Record all the nums have the same digit sum
        let mut set = [[-1; 2]; 90];
        for num in nums {
            // Calculate the digit sum of the number
            let digit_sum = Self::digits_sum(num);
            // Replace the number in set
            if set[digit_sum][0] == -1 {
                set[digit_sum][0] = num;
                continue;
            }
            // Replace the number in set
            if set[digit_sum][1] == -1 {
                set[digit_sum][1] = num;
                continue;
            }

            // Get the right number in every set digit sum
            if set[digit_sum][0] <= set[digit_sum][1] {
                if num > set[digit_sum][0] {
                    set[digit_sum][0] = num;
                }
            } else {
                if num > set[digit_sum][1] {
                    set[digit_sum][1] = num;
                }
            }
        }

        // Get the right max sum
        let mut max_sum = -1;
        for i in 0..90 {
            if set[i][0] != -1 && set[i][1] != -1 {
                max_sum = max_sum.max(set[i][0] + set[i][1]);
            }
        }

        max_sum
    }
}
