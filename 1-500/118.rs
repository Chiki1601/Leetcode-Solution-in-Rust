impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        // 1
        // 1 1
        // 1 2 1
        // 1 3 3 1
        // 1 4 6 4 1
        assert!(num_rows >= 1 && num_rows <= 30);
        let num_rows = num_rows as usize;
        let mut triangle = vec![vec![1]];
        for i in 1..num_rows {
            triangle.push(vec![1; i + 1]);
            for j in 1..i {
                triangle[i][j] = triangle[i - 1][j - 1] + triangle[i - 1][j];
            }
        }
        triangle
    }
}
