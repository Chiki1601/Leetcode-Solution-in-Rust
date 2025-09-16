
impl Solution {
    pub fn replace_non_coprimes(nums: Vec<i32>) -> Vec<i32> {
        let gcd = |mut a: i32, mut b: i32| -> i32 {
            while b != 0 {
                if b < a {
                    std::mem::swap(&mut a, &mut b);
                }
                b %= a;
            }
            a
        };
        let mut result = Vec::with_capacity(nums.len());
        for mut n in nums {
            while let Some(&p) = result.last() {
                match gcd(p, n) {
                    1 => break,
                    gcd => {
                        n *= p / gcd;
                        result.pop();
                    }
                }
            }
            result.push(n);
        }
        result
    }
}
