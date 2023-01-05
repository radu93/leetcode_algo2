struct Solution;

impl Solution {
    fn bin_search_breakpoint(nums: &Vec<i32>, left: i32, right: i32) -> i32 {
        if left > right {
            return -1;
        }

        if left == right {
            return right;
        }

        let m = (left + (right - left) / 2) as usize;

        if nums[m] < nums[right as usize] {
            Solution::bin_search_breakpoint(nums, left, m as i32)
        } else {
            Solution::bin_search_breakpoint(nums, m as i32 + 1, right)
        }
    }

    fn bin_search(nums: &Vec<i32>, target: i32, left: i32, right: i32) -> i32 {
        if left > right {
            return -1;
        }

        let m = (left + (right - left) / 2) as usize;

        if nums[m] == target {
            return m as i32;
        }

        if nums[m] < target {
            Solution::bin_search(nums, target, m as i32 + 1, right)
        } else {
            Solution::bin_search(nums, target, left, m as i32 - 1)
        }
    }

    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut high = nums.len() as i32 - 1;
        let mut low = 0;

        while low < high {

            println!("{low}, {high}");

            let middle = low + (high - low) / 2;
            if nums[middle as usize] < nums[high as usize] {
                high = middle;
            } else {
                low = middle + 1;
            }
        }

        if low == high {
            return nums[high as usize];
        }

        return nums[0];
    }

    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let breakpoint = Solution::bin_search_breakpoint(&nums, 0, nums.len() as i32 - 1);
        if breakpoint == -1 {
            return -1;
        }

        let idx = Solution::bin_search(&nums, target, 0, breakpoint - 1);
        if idx != -1 {
            return idx;
        }
        Solution::bin_search(&nums, target, breakpoint, nums.len() as i32 - 1)
    }
}

fn main() {
    let nums = vec![7, 0, 1, 2, 4, 5];
    println!("{}", Solution::find_min(nums));
}
