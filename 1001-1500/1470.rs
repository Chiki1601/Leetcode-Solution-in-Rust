impl Solution {
    pub fn shuffle(mut nums: Vec<i32>, n: i32) -> Vec<i32> {
        let n = n as usize;
        let mut res = vec![0; 2 * n];
        let mut idx = 0usize;
        for i in 0..n {
            res[idx] = nums[i];
            res[idx+1] = nums[n+i];
            idx+=2;
        }
        res
    }
}
