struct Solution;

impl Solution {
    fn search(
        board: &Vec<Vec<char>>,
        pos: (usize, usize),
        visited: &mut Vec<Vec<bool>>,
        index_covered: usize,
        word: &Vec<char>,
    ) -> bool {

        // println!("{index_covered}, {:?}", pos);

        let (i, j) = pos;
        if word[index_covered] != board[i][j] {
            // println!("bail");
            return false;
        }
        if index_covered == word.len() - 1 {
            // println!("found at {:?}", pos);
            return true;
        }

        let m = board.len();
        let n = board[0].len();

        let mut togo = vec![];

        if i > 0 && !visited[i - 1][j] {
            togo.push((i - 1, j));
        }
        if j > 0 && !visited[i][j - 1] {
            togo.push((i, j - 1));
        }
        if j < n - 1 && !visited[i][j + 1] {
            togo.push((i, j + 1));
        }
        if i < m - 1 && !visited[i + 1][j] {
            togo.push((i + 1, j));
        }

        visited[i][j] = true;

        for next in togo.iter() {
            if Solution::search(board, *next, visited, index_covered + 1, word) {
                return true;
            }
        }

        visited[i][j] = false;
        return false;
    }

    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let m = board.len();
        let n = board[0].len();
        let mut visited = vec![vec![false; n]; m];

        for i in 0..m {
            for j in 0..n {
                // println!("start at {i}, {j}");
                if Solution::search(&board, (i, j), &mut visited, 0, &word.chars().collect()) {
                    return true;
                }
            }
        }
        false
    }
}

fn main() {
    let board0 = vec![
        vec!['A', 'B', 'C', 'E'],
        vec!['S', 'F', 'C', 'S'],
        vec!['A', 'D', 'E', 'E'],
    ];
    let board1 = vec![vec!['a', 'b'], vec!['c', 'd']];
    let word0 = String::from("ABCCED");
    let word1 = String::from("SEE");
    let word2 = String::from("ABCBC");
    let word3 = String::from("abcd");

    println!("{}", Solution::exist(board0, word2));
}
