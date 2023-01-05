struct Solution;

impl Solution {
    fn dfs(board: &Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>, pos: (usize, usize)) {
        if visited[pos.0][pos.1] || board[pos.0][pos.1] != 'O' {
            return;
        }

        visited[pos.0][pos.1] = true;

        if pos.0 as i32 - 1 >= 0 {
            Solution::dfs(board, visited, (pos.0 - 1, pos.1));
        }
        if pos.0 + 1 < board.len() {
            Solution::dfs(board, visited, (pos.0 + 1, pos.1));
        }
        if pos.1 as i32 - 1 >= 0 {
            Solution::dfs(board, visited, (pos.0, pos.1 - 1));
        }
        if pos.1 + 1 < board[0].len() {
            Solution::dfs(board, visited, (pos.0, pos.1 + 1));
        }
    }

    pub fn solve(board: &mut Vec<Vec<char>>) {
        let m = board.len();
        let n = board[0].len();
        let mut visited = vec![vec![false; n]; m];

        for i in 0..m {
            Solution::dfs(board, &mut visited, (i, 0));
            Solution::dfs(board, &mut visited, (i, n - 1));
        }
        for j in 1..n - 1 {
            Solution::dfs(board, &mut visited, (0, j));
            Solution::dfs(board, &mut visited, (m - 1, j));
        }

        for i in 0..m {
            for j in 0..n {
                if !visited[i][j] && board[i][j] == 'O' {
                    board[i][j] = 'X';
                }
            }
        }
    }
}

fn main() {
    let mut board = vec![
        vec!['X', 'X', 'X', 'X'],
        vec!['X', 'O', 'O', 'X'],
        vec!['X', 'X', 'O', 'X'],
        vec!['X', 'O', 'X', 'X'],
    ];
    Solution::solve(&mut board);
    println!("{:?}", board);
}
