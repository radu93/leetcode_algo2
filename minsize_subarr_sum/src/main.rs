struct Solution;

impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut minlen = nums.len() + 1;
        let mut i = 0;
        let mut j = 0;

        while j < nums.len() {
            sum += nums[j];
            while sum - nums[i] >= target && i < j {
                sum -= nums[i];
                i += 1;
            }
            if sum >= target {
                minlen = std::cmp::min(j - i + 1, minlen);
                println!("new min at {i} {j}");
            }
            j += 1;
        }
        if minlen < nums.len() + 1 {
            minlen as i32
        } else {
            0
        }
    }
}

fn main() {
    let nums = vec![1, 1, 1, 1, 1, 1, 1, 1];
    println!("{}", Solution::min_sub_array_len(11, nums));
}
