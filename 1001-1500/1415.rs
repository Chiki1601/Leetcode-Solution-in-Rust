impl Solution {
    pub fn get_happy_string(n: i32, k: i32) -> String {
        fn dfs(curr: String, n: i32, k: i32, counts: &mut i32, res: &mut String) {
            if *counts == k { return }
            if curr.len() as i32 == n {
                *counts += 1;
                if *counts == k {
                    *res = curr; 
                }
                return
            }

            for l in ['a', 'b', 'c'] {
                if curr.len() == 0 || curr.chars().nth(curr.len() - 1) != Some(l) {
                    let v = curr.clone() + &l.to_string();
                    dfs(v, n, k, counts, res);
                }
            }
        }

        let mut counts = 0;
        let mut res = "".to_string();
        dfs("".to_string(), n, k, &mut counts, &mut res);
        res
    }
}
