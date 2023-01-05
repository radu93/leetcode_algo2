use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn shortest_path_binary_matrix(grid: Vec<Vec<i32>>) -> i32 {
        let mut queue = VecDeque::new();
        let m = grid.len();
        let n = grid[0].len();
        let mut lgrid = grid;

        if lgrid[0][0] == 0 {
            queue.push_back((0, 0, 1));
            lgrid[0][0] = 1;
        }

        while !queue.is_empty() {
            if let Some((i, j, dist)) = queue.pop_front() {
                if i == m - 1 && j == n - 1 {
                    return dist;
                }
                if i as i32 - 1 >= 0 {
                    if j as i32 - 1 >= 0 && lgrid[i - 1][j - 1] == 0 {
                        queue.push_back((i - 1, j - 1, dist + 1));
                        lgrid[i - 1][j - 1] = 1;
                    }
                    if lgrid[i - 1][j] == 0 {
                        queue.push_back((i - 1, j, dist + 1));
                        lgrid[i - 1][j] = 1;
                    }
                    if j + 1 < n && lgrid[i - 1][j + 1] == 0 {
                        queue.push_back((i - 1, j + 1, dist + 1));
                        lgrid[i - 1][j + 1] = 1;
                    }
                }
                if i + 1 < m {
                    if j as i32 - 1 >= 0 && lgrid[i + 1][j - 1] == 0 {
                        queue.push_back((i + 1, j - 1, dist + 1));
                        lgrid[i + 1][j - 1] = 1;
                    }
                    if lgrid[i + 1][j] == 0 {
                        queue.push_back((i + 1, j, dist + 1));
                        lgrid[i + 1][j] = 1;
                    }
                    if j + 1 < n && lgrid[i + 1][j + 1] == 0 {
                        queue.push_back((i + 1, j + 1, dist + 1));
                        lgrid[i + 1][j + 1] = 1;
                    }
                }
                if j as i32 - 1 >= 0 && lgrid[i][j - 1] == 0 {
                    queue.push_back((i, j - 1, dist + 1));
                    lgrid[i][j - 1] = 1;
                }
                if j + 1 < n && lgrid[i][j + 1] == 0 {
                    queue.push_back((i, j + 1, dist + 1));
                    lgrid[i][j + 1] = 1;
                }
            }
        }
        -1
    }
}

fn main() {
    let grid0 = vec![vec![0, 0, 0], vec![1, 1, 0], vec![1, 1, 0]];
    let grid1 = vec![vec![0, 1], vec![1, 0]];
    let grid2 = vec![vec![1, 0, 0], vec![1, 1, 0], vec![1, 1, 0]];

    println!("{}", Solution::shortest_path_binary_matrix(grid2));
}
