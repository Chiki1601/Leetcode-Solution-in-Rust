use std::collections::VecDeque;
impl Solution {
    pub fn eventual_safe_nodes(graph: Vec<Vec<i32>>) -> Vec<i32> {
        let mut graph_len = graph.len() as usize;        
        let mut edges: Vec<Vec<usize>> = vec![vec![]; graph_len];
        let mut indegree: Vec<usize> = vec![0; graph_len];
        let mut res = Vec::with_capacity(graph_len);
        
        //  (i, j)
        //  Reversing edges
        //  Track indegree edges
        for i in 0..graph_len { 
            for j in &graph[i] {
                //  i -> j
                edges[*j as usize].push(i);
                indegree[i] +=1;
            }
        }
        //  For Terminal nodes == no outgoing edges or indegree == 0
        let mut queue: VecDeque<usize> = 
        VecDeque::with_capacity(graph_len);
        for i in 0..graph_len { 
            if indegree[i] == 0 { 
                queue.push_back(i)
            }
        }
        //  Simulate top sort: Find all nodes that are not in a cycle 
        while let Some(safe_node) = queue.pop_front() { 
            res.push(safe_node as i32);
            while let Some(cycle_node) = edges[safe_node].pop() { 
                indegree[cycle_node] -= 1;
                if indegree[cycle_node] == 0 { 
                    queue.push_back(cycle_node)
                }
            }
        } 
        res.sort();
        res
    }
}
