struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut low = 0;
        let mut high = height.len() - 1;
        let mut max_area = 0;

        while low < high {
            let h = std::cmp::min(height[low], height[high]);
            let area = (high - low) as i32 * h;

            if area > max_area {
                max_area = area;
            }

            if height[low] < height[high] {
                low += 1;
            } else {
                high -= 1;
            }
        }

        max_area
    }
}

fn main() {
    let heights = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
    println!("{}", Solution::max_area(heights));
}
