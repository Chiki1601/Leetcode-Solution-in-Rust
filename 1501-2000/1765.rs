use std::collections::VecDeque;

impl Solution {
    pub fn highest_peak(is_water: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut is_water = is_water;
        let mut q: VecDeque<(usize, usize)> = VecDeque::new();

        for row_i in 0..is_water.len() {
            for col_i in 0..is_water[0].len() {
                if is_water[row_i][col_i] == 1 {
                    q.push_back((row_i, col_i));
                    is_water[row_i][col_i] = 0;
                } else {
                    is_water[row_i][col_i] = -1;
                }
            }
        }

        let mut iteration = 1;

        while !q.is_empty() {
            let size = q.len();
            for _ in 0..size {
                let (pop_y, pop_x) = q.pop_front().unwrap();

                for (dy, dx) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
                    let new_y = pop_y as isize + dy;
                    let new_x = pop_x as isize + dx;

                    if new_y >= 0
                        && new_x >= 0
                        && new_y < is_water.len() as isize
                        && new_x < is_water[0].len() as isize
                        && is_water[new_y as usize][new_x as usize] == -1
                    {
                        is_water[new_y as usize][new_x as usize] = iteration;
                        q.push_back((new_y as usize, new_x as usize));
                    }
                }
            }
            iteration += 1;
        }

        is_water
    }
}
