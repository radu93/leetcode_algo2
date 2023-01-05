use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let mut slide_map: HashMap<u8, i32> = HashMap::new();
        let mut i = 0;
        let s_bytes = s.as_bytes();
        let p_bytes = p.as_bytes();
        let mut result: Vec<i32> = vec![];

        for b in p_bytes {
            *slide_map.entry(*b).or_default() -= 1;
        }

        for i in 0..s_bytes.len() {
            *slide_map.entry(s_bytes[i]).or_default() += 1;

            let i_start = i as i32 - p_bytes.len() as i32 + 1;
            if i_start < 0 {
                continue;
            }

            if slide_map.values().all(|&x| x == 0) {
                result.push(i_start);
            }

            *slide_map.entry(s_bytes[i_start as usize]).or_default() -= 1;
        }

        result
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::find_anagrams(String::from("abab"), String::from("ab"))
    );
}
