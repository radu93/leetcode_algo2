struct Solution;

impl Solution {
    fn dfs(
        graph: &Vec<Vec<i32>>,
        len: usize,
        node: usize,
        current_path: &mut Vec<i32>,
        paths: &mut Vec<Vec<i32>>,
    ) {
        current_path.push(node as i32);
        if node == len - 1 {
            paths.push(current_path.clone());
        }
        for n in graph[node].iter() {
            Solution::dfs(graph, len, *n as usize, current_path, paths);
        }
        current_path.pop();
    }

    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut paths = vec![];
        let mut current_path = vec![];
        Solution::dfs(&graph, graph.len(), 0, &mut current_path, &mut paths);
        paths
    }
}

fn main() {
    let graph0 = vec![vec![1, 2], vec![3], vec![3], vec![]];
    let graph1 = vec![vec![4, 3, 1], vec![3, 2, 4], vec![3], vec![4], vec![]];
    println!("{:?}", Solution::all_paths_source_target(graph1));
}
