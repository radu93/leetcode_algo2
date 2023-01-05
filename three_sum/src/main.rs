struct Solution;

use std::collections::HashMap;
use std::collections::hash_map::Entry;


impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut arr = nums;
        arr.sort();

        let len = arr.len();
        let mut found = HashMap::new();

        for (pos, elem) in arr.iter().enumerate() {
            let mut low = pos + 1;
            let mut high = len - 1;

            while low <= len - 2 && low < high {
                let sum = elem + arr[low] + arr[high];
                if sum == 0 {
                    found.insert(vec![*elem, arr[low], arr[high]], true);
                    low += 1;
                    high -= 1;
                } else if sum < 0 {
                    low += 1;
                } else {
                    high -= 1;
                }
            }
        }

        found.keys().cloned().collect()
    }
}

fn main() {
    // let nums = vec![-1, 0, 1, 2, -1, -4];
    let nums = vec![-2,0,1,1,2];
    println!("{:?}", Solution::three_sum(nums));
}
