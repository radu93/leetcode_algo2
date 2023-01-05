struct Solution;

impl Solution {
    fn backtrack(
        nums: &Vec<i32>,
        index: usize,
        current_subset: &mut Vec<i32>,
        all_subsets: &mut Vec<Vec<i32>>,
    ) {
        all_subsets.push(current_subset.to_vec());

        for i in index..nums.len() {
            current_subset.push(nums[i]);
            Solution::backtrack(nums, i + 1, current_subset, all_subsets);
            current_subset.pop();
        }
    }

    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut current_subset = vec![];
        let mut all_subsets: Vec<Vec<i32>> = vec![];

        Solution::backtrack(&nums, 0, &mut current_subset, &mut all_subsets);

        all_subsets
    }
}

fn main() {
    let nums = vec![1, 2, 3];
    println!("{:?}", Solution::subsets(nums));
}
