use std::collections::HashMap;
impl Solution {
    pub fn first_complete_index(arr: Vec<i32>, mat: Vec<Vec<i32>>) -> i32 {
        let m: usize = mat.len();
        let n: usize = mat[0].len();
        let mut p: HashMap<i32, (usize, usize)> = HashMap::with_capacity(m*n);
        for i in 0..m {
            for j in 0..n {
                *p.entry(mat[i][j]).or_insert((m, n)) = (i, j);
            }
        }
        let mut rows: Vec<usize> = vec![0; m];
        let mut cols: Vec<usize> = vec![0; n];

        for k in 0..arr.len() {
            if let Some(&(i, j)) = p.get(&arr[k]) {
                rows[i] += 1;
                cols[j] += 1;
                if rows[i] == n || cols[j] == m {
                    return k as i32;
                }
            }
        }
        unreachable!();
    }
}
