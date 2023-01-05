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
            // skip duplicates on this level

            let curr = nums[i];

            if i > index && nums[i - 1] == curr {
                continue;
            }

            current_subset.push(curr);
            Solution::backtrack(nums, i + 1, current_subset, all_subsets);
            current_subset.pop();
        }
    }

    pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut current_subset = vec![];
        let mut all_subsets: Vec<Vec<i32>> = vec![];

        let mut mnums = nums;

        mnums.sort();

        Solution::backtrack(&mnums, 0, &mut current_subset, &mut all_subsets);

        all_subsets
    }
}

fn main() {
    let nums = vec![1, 2, 2];
    println!("{:?}", Solution::subsets_with_dup(nums));
}
