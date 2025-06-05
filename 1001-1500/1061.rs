impl Solution {
    pub fn smallest_equivalent_string(s1: String, s2: String, base_str: String) -> String {
        let n = s1.len();

        let s1: Vec<usize> = s1
            .into_bytes()
            .into_iter()
            .map(|v| (v - 97) as usize)
            .collect();
        let s2: Vec<usize> = s2
            .into_bytes()
            .into_iter()
            .map(|v| (v - 97) as usize)
            .collect();
        let mut adj = [[false; 26]; 26];

        for i in 0..n {
            adj[s1[i]][s2[i]] = true;
            adj[s2[i]][s1[i]] = true;
        }

        let mut map = [0; 26];
        for i in 0..26 {
            map[i as usize] = i;
        }

        let mut visited = [false; 26];

        for c in 0..26 {
            if !visited[c] {
                let mut vec = Vec::new();
                let min = Self::dfs(c, &adj, &mut visited, &mut vec);
                let min = (min + 97) as u8;

                for p in vec {
                    map[p] = min;
                }
            }
        }

        let mut ans = String::new();
        base_str
            .into_bytes()
            .into_iter()
            .for_each(|c| ans.push(map[(c - 97) as usize] as char));
        ans
    }

    fn dfs(n: usize, adj: &[[bool; 26]; 26], v: &mut [bool; 26], vec: &mut Vec<usize>) -> usize {
        let mut min = n;
        vec.push(n);
        v[n] = true;
        for i in 0..26 {
            if adj[i][n] && !v[i] {
                min = min.min(Self::dfs(i, adj, v, vec));
            }
        }
        min
    }
}
