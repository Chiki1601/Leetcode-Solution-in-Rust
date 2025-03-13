impl Solution {
    pub fn min_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
        
        let mut diff_array = vec![0; nums.len() + 1];

        let mut l = 0;
        let mut r = queries.len() + 1;

        while l < r {
            let m = (l + r) / 2;

            if can_zero(&nums, &queries, m, &mut diff_array) {
                r = m;
            } else {
                l = m + 1;
            }
            diff_array.fill(0);
        }
        if l > queries.len() { 
            -1 
        } else { 
            l as i32 
        }
    }
}

fn can_zero(ns: &[i32], qs: &[Vec<i32>], k: usize, da: &mut [i32]) -> bool {

    for q in qs.into_iter().take(k) {
        da[q[0] as usize    ] += q[2];
        da[q[1] as usize + 1] -= q[2];
    }

    let mut max_subtract = 0;

    for (&n, &mut d) in ns.into_iter().zip(da) {
        max_subtract += d;

        if n > max_subtract {
            return false;
        } 
    }
    true
}
