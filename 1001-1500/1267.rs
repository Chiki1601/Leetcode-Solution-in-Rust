impl Solution {
    pub fn count_servers(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let (mut row, mut col) = (vec![0; m], vec![0; n]);

        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 1 {
                    row[i] += 1;
                    col[j] += 1;
                }
            }
        }

        let mut res = 0;

        for i in 0..m {
            for j in 0..n {
                if row[i] > 1 || col[j] > 1 {
                    res += grid[i][j];
                }
            }
        }

        res
    }
}
