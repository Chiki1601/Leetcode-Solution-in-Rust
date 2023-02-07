impl Solution {
    pub fn total_fruit(fruits: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        use std::cmp::max;
        let mut ans:i32 = 0;
        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut left = 0;
        for right in 0..fruits.len() {
            let count = map.entry(fruits[right]).or_insert(0);
            *count += 1;
            while map.len() > 2 {
                map.entry(fruits[left]).and_modify(|x| *x -= 1);
                if *(map.get(&fruits[left]).unwrap()) == 0 {
                    map.remove(&fruits[left]);
                }
                left += 1;
            }
            ans = max(ans, (right - left + 1) as i32);
        }
        ans
    }
}
