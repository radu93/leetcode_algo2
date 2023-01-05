struct Solution;

impl Solution {
    fn dfs(grid: &Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>, i: i32, j: i32) {
        if i < 0 || j < 0 || i >= grid.len() as i32 || j >= grid[0].len() as i32 {
            return;
        }
        if visited[i as usize][j as usize] || grid[i as usize][j as usize] == '0' {
            return;
        }

        visited[i as usize][j as usize] = true;

        Solution::dfs(grid, visited, i, j - 1);
        Solution::dfs(grid, visited, i, j + 1);
        Solution::dfs(grid, visited, i - 1, j);
        Solution::dfs(grid, visited, i + 1, j);
    }

    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut visited = vec![vec![false; n]; m];
        let mut nislands = 0;

        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == '1' && !visited[i][j] {
                    Solution::dfs(&grid, &mut visited, i as i32, j as i32);
                    nislands += 1;
                }
            }
        }
        nislands
    }
}

fn main() {
    let grid0: Vec<Vec<char>> = vec![
        vec!['1', '1', '1', '1', '0'],
        vec!['1', '1', '0', '1', '0'],
        vec!['1', '1', '0', '0', '0'],
        vec!['0', '0', '0', '0', '0'],
    ];
    let grid1: Vec<Vec<char>> = vec![
        vec!['1', '1', '0', '0', '0'],
        vec!['1', '1', '0', '0', '0'],
        vec!['0', '0', '1', '0', '0'],
        vec!['0', '0', '0', '1', '1'],
    ];
    let grid2: Vec<Vec<char>> = vec![
        vec!['0', '0', '0', '0', '0'],
        vec!['0', '0', '0', '0', '0'],
        vec!['0', '0', '0', '0', '0'],
        vec!['0', '0', '0', '0', '0'],
    ];
    println!("{}", Solution::num_islands(grid2));
}
