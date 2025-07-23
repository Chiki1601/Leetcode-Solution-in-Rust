impl Solution {
    pub fn maximum_gain(s: String, x: i32, y: i32) -> i32 {
        let mut ans = 0;
        let mut a = 0;
        let mut b = 0;
        for byte in s.bytes().chain(std::iter::once(b'$')) {
            match byte {
                b'a' => if y >= x && b != 0 {
                        b -= 1;
                        ans += y;
                    }
                    else {
                        a += 1;
                    }
                b'b' => if x >= y && a != 0 {
                        a -= 1;
                        ans += x;
                    }
                    else {
                        b += 1;
                    }
                _ => {
                    ans += a.min(b) * x.min(y);
                    a = 0;
                    b = 0;
                }
            }
        }
        ans
    }
}
