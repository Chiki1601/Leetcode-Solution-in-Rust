impl Solution {
    pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        let mut adj = vec![Vec::new(); n as usize];
        for v in flights.iter() {
            adj[v[0] as usize].push((v[1] as usize, v[2]));
        }
        let mut cost = vec![i32::MAX; n as usize];
        cost[src as usize] = 0;
        for _ in 0..=k {
            let mut cost2 = cost.clone();
            for i in 0..n as usize {
                if cost[i] != i32::MAX {
                    for &(j, price) in adj[i].iter() {
                        cost2[j] = cost2[j].min(cost[i] + price);
                    }
                }
            }
            cost = cost2;
        }
        match cost[dst as usize] {
            i32::MAX => -1,
            x => x,
        }
    }
}
