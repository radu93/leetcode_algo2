struct Solution;

impl Solution {
    fn backtrack(
        candidates: &Vec<i32>,
        target: i32,
        index: usize,
        current: &mut Vec<i32>,
        current_sum: i32,
        ans: &mut Vec<Vec<i32>>,
    ) {
        if current_sum == target {
            ans.push(current.to_vec());
            return;
        }
        if current_sum > target {
            return;
        }

        for i in index..candidates.len() {
            // skip duplicates for this level
            if i > index && candidates[i] == candidates[i - 1] {
                continue;
            }
            current.push(candidates[i]);
            Solution::backtrack(
                candidates,
                target,
                i + 1,
                current,
                current_sum + candidates[i],
                ans,
            );
            current.pop();
        }
    }

    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut current = vec![];
        let mut ans = vec![];

        let mut mcandidates = candidates;

        mcandidates.sort();

        Solution::backtrack(&mcandidates, target, 0, &mut current, 0, &mut ans);

        ans
    }
}

fn main() {
    let candidates = vec![10, 1, 2, 7, 6, 1, 5];
    println!("{:?}", Solution::combination_sum2(candidates, 8));
}