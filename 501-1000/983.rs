impl Solution {
    pub fn mincost_tickets(days: Vec<i32>, costs: Vec<i32>) -> i32 {
        let last = days[days.len() - 1] as usize;
        let mut min_cost = [0; 366];
        let mut is_travel = [false; 366];
        for &day in &days {
            is_travel[day as usize] = true;
        }
        for i in 1..=last as i32 {
            if !is_travel[i as usize] {
                min_cost[i as usize] = min_cost[i as usize - 1];
                continue;
            }
            let mut min = min_cost[i as usize - 1] + costs[0];
            min = std::cmp::min(min, min_cost[std::cmp::max(i - 7, 0) as usize] + costs[1]);
            min = std::cmp::min(min, min_cost[std::cmp::max(i - 30, 0) as usize] + costs[2]);
            min_cost[i as usize] = min;
        }
        min_cost[last]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mincost_tickets() {
        assert_eq!(
            Solution::mincost_tickets(vec![1, 4, 6, 7, 8, 20], vec![2, 7, 15]),
            11
        )
    }

    #[test]
    fn test_mincost_tickets_02() {
        assert_eq!(
            Solution::mincost_tickets(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 30, 31], vec![2, 7, 15]),
            17
        )
    }
}
