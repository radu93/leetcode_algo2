struct Solution;

impl Solution {
    fn is_peak(nums: &Vec<i32>, index: usize) -> bool {
        let mut gt_right = true;
        let mut gt_left = true;

        if let Some(right_elem) = nums.get(index + 1) {
            gt_right = nums[index] > *right_elem;
        }
        
        
        if let Some(left_elem) = nums.get(index - 1) {
            gt_left = nums[index] > *left_elem;
        }

        gt_left && gt_right
    }

    fn bin_search_peak(nums: &Vec<i32>, low: i32, high: i32) -> i32 {

        println!("{low}, {high}");

        let middle = low + (high - low) / 2;

        if Solution::is_peak(nums, middle as usize) {
            return middle;
        }

        if low >= high {
            println!("should never get here");
            return -1;
        }

        let mut gt_right = true;
        let mut gt_left = true;

        if let Some(right_elem) = nums.get(middle as usize + 1) {
            if nums[middle as usize] < *right_elem {
                gt_right = false;          
            }
        }

        if let Some(left_elem) = nums.get(middle as usize - 1) {
            if nums[middle as usize] < *left_elem {
                gt_left = false;          
            }
        }

        if gt_left {
            Solution::bin_search_peak(nums, middle + 1, high)
        } else {
            Solution::bin_search_peak(nums, low, middle - 1)
        }
        
    }

    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        Solution::bin_search_peak(&nums, 0, nums.len() as i32 - 1)
    }

}

fn main() {
    let nums = vec![1, 2, 1, 3, 5, 6, 4];
    println!("{}", Solution::find_peak_element(nums));
}
