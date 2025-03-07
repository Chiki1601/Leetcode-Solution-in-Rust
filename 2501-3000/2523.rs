impl Solution {
    pub fn closest_primes(left: i32, right: i32) -> Vec<i32> {
        let mut primes = vec![true; right as usize + 1];
        primes[0] = false;
        primes[1] = false;

        let mut i = 2;

        while i * i <= right as usize {
            if !primes[i] {
                i += 1;
                continue;
            }

            let mut cur = i * i;

            while cur <= right as usize {
                primes[cur] = false;
                cur += i;
            }

            i += 1;
        }

        primes
            .into_iter()
            .enumerate()
            .skip(left as usize)
            .take(right as usize + 1 - left as usize)
            .filter(|&(_, x)| x)
            .map(|(i, _)| i as i32)
            .collect::<Vec<_>>()
            .windows(2)
            .fold(
                (vec![-1, -1], i32::MAX),
                |(mut result, mut min), x| {
                    let cur = x[1] - x[0];

                    if cur < min {
                        result.copy_from_slice(x);
                        min = cur;
                    }

                    (result, min)
                },
            )
            .0
    }
}
