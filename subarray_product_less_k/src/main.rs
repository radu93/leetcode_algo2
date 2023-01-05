struct Solution;

impl Solution {
    pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
        let mut result = 0;

        let mut left: i32 = 0;

        let mut prod = 1;

        for (right, elem) in nums.iter().enumerate() {
            prod *= *elem;
            while prod >= k && left <= right as i32 {
                prod /= nums[left as usize];
                left += 1;
            }
            result += right as i32 - left + 1;
        }

        result
    }
}

fn main() {
    println!(
        "{}",
        Solution::num_subarray_product_less_than_k(vec![1, 1, 1, 1], 0)
    );
}
