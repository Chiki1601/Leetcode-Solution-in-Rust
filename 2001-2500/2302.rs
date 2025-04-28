impl Solution {
    pub fn count_subarrays(mut nums: Vec<i32>, k: i64) -> i64 {
        let mut p_sum = vec![0_i64];
        
        for i in 0..nums.len(){
            p_sum.push((nums[i] as i64) + p_sum[i]);
        }
        
        let mut cnt = 0;
        
        let mut i = 0;
        for j in 1..p_sum.len(){
            
            while (p_sum[j] - p_sum[i]) * ((j - i) as i64) >= k{
                i+=1;
            }
            
            cnt += j - i;
        }
        
        cnt as i64
    }
}
