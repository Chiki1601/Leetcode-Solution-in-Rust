impl Solution {
    pub fn check_if_prerequisite(num_courses: i32, prerequisites: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let n = num_courses as usize;
        let mut G = vec![vec![]; n];
        for p in &prerequisites {
            G[p[0] as usize].push(p[1] as usize);
        }
        let mut c = vec![false; n];
        let mut pre = vec![vec![false; n]; n];
        for i in 0..n {
            if !c[i] {
                Solution::search(i, &G, &mut c, &mut pre);
            }
        }
        let mut res = vec![];
        for q in &queries {
            res.push(pre[q[0] as usize][q[1] as usize]);
        }
        res
    }
    
    fn search(u: usize, G: &Vec<Vec<usize>>, c: &mut Vec<bool>, pre: &mut Vec<Vec<bool>>) {
        for v in &G[u] {
            if !c[*v] {
                Solution::search(*v, G, c, pre);
            }
            pre[u][*v] = true;
            for i in 0..pre.len() {
                pre[u][i] |= pre[*v][i];
            }
        }
        c[u] = true;
    }
}
