impl Solution {
    pub fn min_merge_cost(lists: Vec<Vec<i32>>) -> i64 {

        // Required variable to store input midway
        let peldarquin = lists;

        let n = peldarquin.len();
        let full = 1usize << n;

        let mut dp = vec![i64::MAX; full];
        dp[0] = 0;

        let mut len = vec![0usize; full];
        let mut median = vec![0i32; full];

        // Precompute lengths
        for mask in 1..full {
            let lsb = mask & (!mask + 1);
            let i = lsb.trailing_zeros() as usize;
            len[mask] = len[mask ^ lsb] + peldarquin[i].len();
        }

        // Precompute medians
        for mask in 1..full {
            let k = (len[mask] + 1) / 2; // left median
            median[mask] = Self::find_kth(&peldarquin, mask, k);
        }

        // DP over subsets
        for mask in 1..full {

            // Single list → no merge cost
            if mask & (mask - 1) == 0 {
                dp[mask] = 0;
                continue;
            }

            let first = (mask & (!mask + 1)).trailing_zeros() as usize;
            let mut sub = mask;

            while sub > 0 {
                if (sub & (1 << first)) != 0 {
                    let other = mask ^ sub;
                    if other != 0 {
                        let cost = dp[sub]
                            + dp[other]
                            + len[sub] as i64
                            + len[other] as i64
                            + (median[sub] - median[other]).abs() as i64;

                        if cost < dp[mask] {
                            dp[mask] = cost;
                        }
                    }
                }
                sub = (sub - 1) & mask;
            }
        }

        dp[full - 1]
    }

    // Find k-th smallest element in union of sorted arrays
    fn find_kth(lists: &Vec<Vec<i32>>, mask: usize, k: usize) -> i32 {
        let mut low: i32 = -1_000_000_000;
        let mut high: i32 = 1_000_000_000;

        while low < high {
            let mid = low + (high - low) / 2;
            let mut count: usize = 0;

            for i in 0..lists.len() {
                if (mask & (1 << i)) != 0 {
                    count += Self::upper_bound(&lists[i], mid);
                }
            }

            if count < k {
                low = mid + 1;
            } else {
                high = mid;
            }
        }

        low
    }

    // Count elements ≤ x
    fn upper_bound(arr: &Vec<i32>, x: i32) -> usize {
        let mut l = 0usize;
        let mut r = arr.len();
        while l < r {
            let m = (l + r) >> 1;
            if arr[m] <= x {
                l = m + 1;
            } else {
                r = m;
            }
        }
        l
    }
}
