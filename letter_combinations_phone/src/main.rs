use std::collections::HashMap;

struct Solution;

impl Solution {
    fn backtrack(
        digits: &[u8],
        map: &HashMap<u8, &str>,
        index: usize,
        current: &mut Vec<u8>,
        ans: &mut Vec<String>,
    ) {
        if index == digits.len() {
            if let Ok(s) = std::str::from_utf8(current) {
                ans.push(s.to_string());
            }
            return;
        }
        
        if let Some(letters) = map.get(&digits[index]) {
            for l in letters.chars() {
                current.push(l as u8);
                Solution::backtrack(digits, map, index + 1, current, ans);
                current.pop();
            }
        }
    }

    pub fn letter_combinations(digits: String) -> Vec<String> {
        let map = HashMap::from([
            ('2' as u8, "abc"),
            ('3' as u8, "def"),
            ('4' as u8, "ghi"),
            ('5' as u8, "jkl"),
            ('6' as u8, "mno"),
            ('7' as u8, "pqrs"),
            ('8' as u8, "tuv"),
            ('9' as u8, "wxyz"),
        ]);
        let byte_digits = digits.as_bytes();
        let mut current = vec![];
        let mut ans = vec![];

        if byte_digits.len() >  0 {
            Solution::backtrack(byte_digits, &map, 0, &mut current, &mut ans);
        }
        ans
    }
}

fn main() {
    let digits = "";
    println!("{:?}", Solution::letter_combinations(digits.to_string()));
}
