impl Solution {
    pub fn max_free_time(event_time: i32, k: i32, start_time: Vec<i32>, end_time: Vec<i32>) -> i32 {
        let n = start_time.len();
        let k = k as usize;
        let mut res = 0;
        let mut sum = vec![0; n + 1];
        for i in 0..n {
            sum[i + 1] = sum[i] + end_time[i] - start_time[i];
        }
        for i in (k - 1)..n {
            let right = if i == n - 1 { event_time } else { start_time[i + 1] };
            let left = if i == k - 1 { 0 } else { end_time[i - k] };
            res = res.max(right - left - (sum[i + 1] - sum[i - k + 1]));
        }
        res
    }
}
