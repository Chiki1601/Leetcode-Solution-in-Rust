use std::{cmp::Reverse, collections::BinaryHeap, mem::replace};
impl Solution {
    pub fn min_time_to_reach(mut move_time: Vec<Vec<i32>>) -> i32 {
        move_time[0][0] = -1;
        let (rows, cols) = (move_time.len() - 1, move_time[0].len() - 1);
        let mut queue = BinaryHeap::from([(Reverse(0), 0, 0)]);
        while let Some((Reverse(mt), y, x)) = queue.pop() {
            if y == rows && x == cols {
                return mt;
            }
            for (dy, dx) in [(y.wrapping_sub(1), x), (y + 1, x), (y, x.wrapping_sub(1)), (y, x + 1)] {
                if dy > rows || dx > cols || move_time[dy][dx] == -1 {
                    continue;
                }
                queue.push((Reverse(replace(&mut move_time[dy][dx], -1).max(mt) + 1), dy, dx));
            }
        }
        unreachable!()
    }
}
