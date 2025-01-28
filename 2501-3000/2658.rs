impl Solution {
  pub fn find_max_fish(M: Vec<Vec<i32>>) -> i32 {
    const DIR: [i32; 5] = [0, 1, 0, -1, 0];
    let (Y, X) = (M.len(), M[0].len());
    let mut seen = vec![vec![false; X]; Y];
    let mut res = 0;

    for y in 0 .. Y {
      for x in 0 .. X {
        if M[y][x] == 0 || seen[y][x] { continue; }

        let mut frontier = vec![[y as i32, x as i32]];
        let mut cnt = M[y][x];
        seen[y][x] = true;
        
        while !frontier.is_empty() {
          let mut new_frontier = Vec::new();
          for v in frontier {
            for i in 0 .. 4 {
              let (y_, x_) = (v[0] + DIR[i], v[1] + DIR[i + 1]);
              if y_ < 0 || x_ < 0 { continue }
              
              let (y_, x_) = (y_ as usize, x_ as usize);
              if y_ < Y && x_ < X && M[y_][x_] > 0 && !seen[y_][x_] {
                seen[y_][x_] = true;
                cnt += M[y_][x_];
                new_frontier.push([y_ as i32, x_ as i32]);
              }
            }
          }

          frontier = new_frontier;
        }
        res = res.max(cnt);
      }
    }

    return res;
  }
}
