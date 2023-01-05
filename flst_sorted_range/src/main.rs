fn search_range_dummy(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut range: Vec<i32> = vec![-1, -1];
    let mut pending = false;

    for (i, n) in nums.iter().enumerate() {
        if *n == target && range[0] == -1 {
            range[0] = i as i32;
            pending = true;
        } else if pending && *n != target {
            pending = false;
            range[1] = (i - 1) as i32;
        }
    }

    if range[0] != -1 && range[1] == -1 {
        range[1] = if pending { nums.len() as i32 } else { range[0] }
    }

    range
}

fn low_bin_search(nums: &Vec<i32>, target: i32, left: i32, right: i32) -> i32 {
    println!("left: {}, right: {}", left, right);

    if left == right {
        return if nums[left as usize] == target {
            left
        } else {
            -1
        };
    }

    let m = left + (right - left) / 2;

    if nums[m as usize] >= target {
        low_bin_search(nums, target, left, m)
    } else {
        low_bin_search(nums, target, m + 1, right)
    }
}

fn high_bin_search(nums: &Vec<i32>, target: i32, left: i32, right: i32) -> i32 {
    println!("left: {}, right: {}", left, right);

    if left == right {
        return if nums[left as usize] == target {
            left
        } else {
            -1
        };
    }

    let m = left + (right - left + 1) / 2;

    if nums[m as usize] <= target {
        high_bin_search(nums, target, m, right)
    } else {
        high_bin_search(nums, target, left, m - 1)
    }
}

fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    if nums.len() == 0 {
        return vec![-1, -1];
    }
    let mut range: Vec<i32> = vec![
        low_bin_search(&nums, target, 0, nums.len() as i32 - 1),
        high_bin_search(&nums, target, 0, nums.len() as i32 - 1),
    ];
    range
}

fn main() {
    let nums = vec![];
    let rng = search_range(nums, 1);

    for n in &rng {
        println!("{n}");
    }
}
