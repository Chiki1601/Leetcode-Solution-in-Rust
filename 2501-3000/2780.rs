impl Solution {
    pub fn minimum_index(nums: Vec<i32>) -> i32 {
        fn find_dominant_element(arr: &Vec<i32>) -> Option<i32> {
            let mut candidate = None;
            let mut count = 0;

            // Boyer-Moore Majority Vote algorithm
            for &num in arr.iter() {
                if count == 0 {
                    candidate = Some(num);
                    count = 1;
                } else if Some(num) == candidate {
                    count += 1;
                } else {
                    count -= 1;
                }
            }

            if let Some(cand) = candidate {
                let total_count = arr.iter().filter(|&&num| num == cand).count();
                if total_count > arr.len() / 2 {
                    return Some(cand);
                }
            }

            None
        }

        let dominant = match find_dominant_element(&nums) {
            Some(val) => val,
            None => return -1,
        };

        let mut left_count = 0;
        let total_dominant_count = nums.iter().filter(|&&num| num == dominant).count();

        for i in 0..nums.len() - 1 {
            if nums[i] == dominant {
                left_count += 1;
            }

            let left_subarray_count = left_count;
            let right_subarray_count = total_dominant_count - left_count;

            if left_subarray_count > (i + 1) / 2 && right_subarray_count > (nums.len() - i - 1) / 2 {
                return i as i32;
            }
        }

        -1
    }
}
