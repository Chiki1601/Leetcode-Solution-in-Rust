impl Solution {
    pub fn find_different_binary_string(nums: Vec<String>) -> String {
        use std::collections::HashSet;
        
        let length = nums[0].len() as u32;
        let mut hs = HashSet::new();
        
        nums
            .iter()
            .for_each(|x| {
                hs.insert(isize::from_str_radix(x, 2).unwrap());
            });
        
            
        for x in 0..isize::pow(2, length) {
            if !hs.contains(&x) {
                let result = format!("{:b}", x);
                return "0".repeat(length as usize - result.len() as usize) + &result
            }
        }
        
        panic!("Should not reach here.");
    }
}
