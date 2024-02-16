impl Solution {
    pub fn find_least_num_of_unique_ints(arr: Vec<i32>, mut k: i32) -> i32 {
        use std::collections::HashMap;
        let mut freq = HashMap::new();
        for n in arr {
            *freq.entry(n).or_insert(0) += 1;
        }
        let mut ordered = freq.values().copied().collect::<Vec<_>>();
        ordered.sort();
        ordered.reverse();
        while k > 0 {
            if *ordered.last().unwrap() <= k {
                k -= *ordered.last().unwrap();
                ordered.pop();
            } else {
                break;
            }
        }
        ordered.len() as i32        
    }
}
