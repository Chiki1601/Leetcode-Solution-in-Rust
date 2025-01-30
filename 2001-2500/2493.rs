use std::collections::VecDeque;

impl Solution {
    pub fn magnificent_sets(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut graph = vec![vec![]; n];

        // Build adjacency list representation of the graph
        for edge in edges {
            let (u, v) = ((edge[0] - 1) as usize, (edge[1] - 1) as usize);
            graph[u].push(v);
            graph[v].push(u);
        }

        let mut heights = vec![None; n]; // Store the maximum height for each component
        let mut total_diameters = 0;

        // Iterate over all nodes, attempting to maximize height for each connected component
        for start in 0..n {
            match Self::bfs_check_and_find_height(start, &graph, &mut heights) {
                (false, _) => return -1,  // If a component is not bipartite, return -1
                (true, Some(d)) => total_diameters += d, // Add the largest diameter found
                _ => {} // No valid height update, move to the next node
            }
        }

        total_diameters
    }

    // Performs BFS from `start` to determine if the graph is bipartite
    // and find the maximum height in its connected component.
    fn bfs_check_and_find_height(start: usize, graph: &[Vec<usize>], heights: &mut [Option<i32>]) -> (bool, Option<i32>) {
        let mut level = vec![0 as i32; graph.len()]; // Track BFS levels
        let mut queue = VecDeque::new();
        queue.push_back(start);
        level[start] = 1;

        let mut max_height = -1; // Track the maximum assigned height
        let mut max_level = 1;   // Track the deepest BFS level reached
        let mut all_heights_assigned = true; // Check if all nodes have valid heights

        while let Some(v) = queue.pop_front() {
            max_level = level[v]; // Update the deepest level reached

            // If a node is unprocessed, mark heights as incomplete
            if v != start && heights[v].is_none() {
                all_heights_assigned = false;
            } else {
                max_height = max_height.max(heights[v].unwrap_or(-1));
            }

            // Traverse neighbors and check bipartiteness
            for &u in &graph[v] {
                if level[u] == 0 { // First visit to `u`
                    level[u] = level[v] + 1;
                    queue.push_back(u);
                } else if level[u].abs_diff(level[v]) != 1 { // Bipartiteness check
                    return (false, None); // Graph is not bipartite
                }
            }
        }

        heights[start] = Some(max_level); // Assign max BFS level as height for `start`
        
        // If all nodes had assigned heights, return the max found height
        if all_heights_assigned {
            (true, Some(max_height.max(max_level)))
        } else {
            (true, None)
        }
    }
}
