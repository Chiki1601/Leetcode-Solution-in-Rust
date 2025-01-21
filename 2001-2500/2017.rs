impl Solution {
    pub fn grid_game(grid: Vec<Vec<i32>>) -> i64 {
        grid[0]
            .iter()
            .zip(&grid[1])
            .scan(
                (grid[0].iter().map(|&num| num as i64).sum(), 0),
                |(sum1, sum2), (&num1, &num2)| {
                    if *sum1 < 0 {
                        return None;
                    }

                    *sum1 -= num1 as i64;
                    if *sum1 > *sum2 {
                        *sum2 += num2 as i64;
                        Some(*sum1)
                    } else {
                        *sum1 = -1;
                        Some(*sum2)
                    }
                },
            )
            .min()
            .unwrap()
    }
}
