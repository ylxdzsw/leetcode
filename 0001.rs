// Given an array of integers, return indices of the two numbers such that they add up to a specific target.

// You may assume that each input would have exactly one solution, and you may not use the same element twice.

// Example:

// Given nums = [2, 7, 11, 15], target = 9,

// Because nums[0] + nums[1] = 2 + 7 = 9,
// return [0, 1].

use std::collections::BTreeMap;

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut complements: BTreeMap<i32, i32> = BTreeMap::new(); // complement -> index

    for (i, n) in nums.iter().enumerate() {
        let cp = target - n;
        if let Some(j) = complements.get(&cp) {
            return vec![*j, i as _]
        }
        complements.insert(*n, i as _);
    }

    unreachable!() // the solution exists
}

fn main() {
    assert!(two_sum(vec![2, 7, 11, 15], 9) == vec![0, 1])
}

// faster than 100.00%, less memory than 100.00%