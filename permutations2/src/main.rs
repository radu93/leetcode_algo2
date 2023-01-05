use std::collections::HashSet;

struct Solution;

impl Solution {
    fn backtrack(index: usize, current: &mut Vec<i32>, permutations: &mut Vec<Vec<i32>>) {
        if index == current.len() {
            permutations.push(current.to_vec());
            println!("saving {:?}", current);
            return;
        }

        let mut used = HashSet::new();

        for i in index..current.len() {

            if used.contains(&current[i]) {
                continue;
            }
            
            used.insert(current[i]);

            current.swap(i, index);
            println!("swapping {} with {} => {:?}", index, i, current);
            Solution::backtrack(index + 1, current, permutations);
            current.swap(i, index);
            println!("swapping back => {:?}", current);
        }
    }

    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut permutations = vec![];
        let mut current = nums;

        Solution::backtrack(0, &mut current, &mut permutations);
        permutations
    }
}

fn main() {
    let nums = vec![0, 1, 0, 0, 9];
    println!("{:?}", Solution::permute_unique(nums));
}
