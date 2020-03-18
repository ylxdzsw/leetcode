// Given an array nums of n integers, are there elements a, b, c in nums such that a + b + c = 0? Find all unique triplets in the array which gives the sum of zero.

// Note:

// The solution set must not contain duplicate triplets.

// Example:

// Given array nums = [-1, 0, 1, 2, -1, -4],

// A solution set is:
// [
//   [-1, 0, 1],
//   [-1, -1, 2]
// ]

#[allow(clippy::unreadable_literal)]
fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut nums = nums;
    nums.sort_unstable();

    let mut result = vec![];
    let mut last_vi = -393939; // TODO: use nullable pointer instead
    let mut last_vj = -393939; 
    for (i, vi) in nums.iter().copied().enumerate() {
        if vi == last_vi { continue }
        last_vi = vi;
        for (j, vj) in nums[i+1..].iter().copied().enumerate() {
            if vj == last_vj { continue }
            last_vj = vj;
            if let Ok(k) = nums[i+j+2..].binary_search(&(-vi-vj)) {
                result.push(vec![vi, vj, nums[i+j+2+k]])
            }
        }
    }

    result
}

fn main() {
    let mut x = three_sum(vec![-1, 0, 1, 2, -1, -4]);
    for l in x.iter_mut() {
        l.sort()
    }
    x.sort();
    assert_eq!(x, &[&[-1, -1, 2], &[-1, 0, 1]]);
}

// faster than 61.45%, less memory than 100.00%.
// A HashSet might help, but we need to ensure not to use an element twice.