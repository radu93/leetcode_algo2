struct Solution;

impl Solution {
    fn dfs(is_connected: &Vec<Vec<i32>>, visited: &mut Vec<bool>, start: usize) {
        let n = visited.len();

        visited[start] = true;

        for i in 0..n { 
            if is_connected[start][i] == 1 && !visited[i] {
                Solution::dfs(is_connected, visited, i);
            }
        }
    }

    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        let n = is_connected.len();
        let mut visited = vec![false; n];
        let mut num_provs = 0;

        for start in 0..n {
            if !visited[start] {
                num_provs += 1;
                Solution::dfs(&is_connected, &mut visited, start);
            }
        }

        num_provs
    }
}

fn main() {
    let adj = vec![vec![1,1,0],vec![1,1,0],vec![0,0,1]];
    println!("{}", Solution::find_circle_num(adj));
}
