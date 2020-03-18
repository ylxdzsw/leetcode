// Given an array nums of n integers and an integer target, find three integers in nums such that the sum is closest to target. Return the sum of the three integers. You may assume that each input would have exactly one solution.

// Example:

// Given array nums = [-1, 2, 1, -4], and target = 1.

// The sum that is closest to the target is 2. (-1 + 2 + 1 = 2).

fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
    let mut nums = nums;
    nums.sort_unstable();

    let mut best = 10086; // TODO?
    for (i, vi) in nums.iter().copied().enumerate() {
        for (j, vj) in nums[i+1..].iter().copied().enumerate() {
            if let Err(k) = nums[i+j+2..].binary_search(&(target-vi-vj)) {
                if k != 0 {
                    let v = nums[i+j+1+k] + vi + vj;
                    if (target - v).abs() < (target - best).abs() {
                        best = v
                    }
                }
                if i+j+2+k < nums.len() {
                    let v = nums[i+j+2+k] + vi + vj;
                    if (target - v).abs() < (target - best).abs() {
                        best = v
                    }
                }
            } else {
                return target
            }
        }
    }

    best
}

fn main() {
    assert_eq!(three_sum_closest(vec![-1, 2, 1, -4], 1), 2);
}

// faster than 100.00%, less memory than 100.00%.