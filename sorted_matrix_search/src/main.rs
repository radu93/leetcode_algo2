struct Solution;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let m = matrix.len();
        if m == 0 {
            return false;
        }
        let n = matrix[0].len();
        if n == 0 {
            return false;
        }

        // find row
        let mut low: i32 = 0;
        let mut high: i32 = m as i32 - 1;

        while low <= high {
            let middle = low + (high - low) / 2;
            if matrix[middle as usize][n - 1] == target { 
                return true;
            }
            if matrix[middle as usize][n - 1] < target {
                low = middle + 1;
            } else {
                high = middle - 1;
            }
        }

        let row = (high + 1) as usize;

        if row >= m {
            return false;
        }

        low = 0;
        high = n as i32 - 1;

        // find value in row
        while low <= high {
            let middle = low + (high - low) / 2;
            if matrix[row][middle as usize] == target { 
                return true;
            }
            if matrix[row][middle as usize] < target {
                low = middle + 1;
            } else {
                high = middle - 1;
            }
        }

        false
    }
}

fn main() {
    let matrix = vec![vec![1, 3, 5, 7], vec![23, 30, 34, 60]];

    println!("{:?}", matrix);

    println!("{}", Solution::search_matrix(matrix, 34));
}
