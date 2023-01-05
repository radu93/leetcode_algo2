struct Solution;

impl Solution {
    fn backtrack(n: i32, nopen: i32, current: &mut Vec<u8>, ans: &mut Vec<String>) {
        if current.len() as i32 == 2 * n {
            if nopen == 0 {
                if let Ok(s) = std::str::from_utf8(current) {
                    ans.push(s.to_string());
                }
            }
            return;
        }
        if nopen > 0 {
            current.push(')' as u8);
            Solution::backtrack(n, nopen - 1, current, ans);
            current.pop();
        }

        current.push('(' as u8);
        Solution::backtrack(n, nopen + 1, current, ans);
        current.pop();
    }

    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut current = vec![];
        let mut ans = vec![];
        Solution::backtrack(n, 0, &mut current, &mut ans);
        ans
    }
}

fn main() {
    println!("{:?}", Solution::generate_parenthesis(3));
}
