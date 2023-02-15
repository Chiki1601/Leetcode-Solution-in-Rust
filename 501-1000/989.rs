impl Solution {
    pub fn add_to_array_form(num: Vec<i32>, mut k: i32) -> Vec<i32> {
        let mut carry = 0;
        let mut a = num.into_iter().rev();
        let mut b = 0;
        let mut ans = Vec::new();

        loop {
            let ta = a.next();
            b = k % 10;
            k /= 10;
            if ta.is_none() && k == 0 && b == 0 {
                break;
            }

            carry += ta.unwrap_or(0) + b;
            ans.push(carry % 10);
            carry /= 10;
        }
        if carry == 1 {
            ans.push(1);
        }
        ans.reverse();
        ans
    }
}
