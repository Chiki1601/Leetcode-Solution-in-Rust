impl Solution {
    pub fn query_results(limit: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ball_colors = HashMap::new();
        let mut color_counts = HashMap::new();

        queries
            .into_iter()
            .map(move |query| {
                match ball_colors.entry(query[0]) {
                    Entry::Vacant(ball_color) => {
                        ball_color.insert(query[1]);
                        let color_count = color_counts.entry(query[1]).or_insert(0);
                        *color_count += 1;
                    }
                    Entry::Occupied(mut ball_color) => {
                        let old_color = std::mem::replace(ball_color.get_mut(), query[1]);
                        if old_color != query[1] {
                            let color_count = color_counts.entry(query[1]).or_insert(0);
                            *color_count += 1;

                            let Entry::Occupied(mut color_count) = color_counts.entry(old_color)
                            else {
                                unreachable!("the color should exist");
                            };
                            *color_count.get_mut() -= 1;
                            if *color_count.get() == 0 {
                                color_count.remove();
                            }
                        }
                    }
                }

                color_counts.len() as i32
            })
            .collect()
    }
}

use std::collections::{hash_map::Entry, HashMap};
