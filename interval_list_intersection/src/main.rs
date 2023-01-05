struct Solution;

impl Solution {
    fn intersect(i1: &Vec<i32>, i2: &Vec<i32>) -> Option<Vec<i32>> {
        let left = std::cmp::max(i1[0], i2[0]);
        let right = std::cmp::min(i1[1], i2[1]);

        if left <= right {
            return Some(vec![left, right]);
        } else {
            return None;
        }
    }

    pub fn interval_intersection(
        first_list: Vec<Vec<i32>>,
        second_list: Vec<Vec<i32>>,
    ) -> Vec<Vec<i32>> {
        let mut intersections = vec![];
        let mut i = 0;
        let mut j = 0;

        while i < first_list.len() && j < second_list.len() {
            let i1 = &first_list[i];
            let i2 = &second_list[j];

            if let Some(intersection) = Solution::intersect(i1, i2) {
                intersections.push(intersection);
            }

            if i1[1] < i2[1] {
                i += 1;
            } else {
                j += 1;
            }
        }

        intersections
    }
}

fn main() {
    let l1 = vec![vec![0, 2], vec![5, 10], vec![13, 23], vec![24, 25]];
    let l2 = vec![vec![1, 5], vec![8, 12], vec![15, 24], vec![25, 26]];
    let l = Solution::interval_intersection(l1, l2);
    println!("{:?}", l);
}
