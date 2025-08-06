impl Solution {
    pub fn num_of_unplaced_fruits(fruits: Vec<i32>, baskets: Vec<i32>) -> i32 {
        let n = baskets.len();
        let mut N = 1;
        while N <= n { N <<= 1; }

        let mut seg = vec![0; N << 1];
        for i in 0..n {
            seg[N + i] = baskets[i];
        }

        for i in (1..N).rev() {
            seg[i] = seg[2 * i].max(seg[2 * i + 1]);
        }

        let mut count = 0;

        for &fruit in &fruits {
            let mut idx = 1;
            if seg[idx] < fruit {
                count += 1;
                continue;
            }

            while idx < N {
                if seg[2 * idx] >= fruit {
                    idx = 2 * idx;
                } else {
                    idx = 2 * idx + 1;
                }
            }

            seg[idx] = -1;
            let mut temp = idx;
            while temp > 1 {
                temp >>= 1;
                seg[temp] = seg[2 * temp].max(seg[2 * temp + 1]);
            }
        }

        count
    }
}
