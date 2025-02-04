impl Solution {
    pub fn max_ascending_sum(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut last_num = 0;
        let mut high_sum = 0;

        for num in nums {
            if num > last_num {
                sum = sum + num;
            }else if num <= last_num{
                if sum > high_sum {
                    high_sum = sum;
                }
                sum = num;

            }

            last_num = num;
        }
        if sum > high_sum {
                    high_sum = sum;
        }
        high_sum

    }
}
