// Given an unsorted integer array, find the smallest missing positive integer.

// Example 1:

// Input: [1,2,0]
// Output: 3
// Example 2:

// Input: [3,4,-1,1]
// Output: 2
// Example 3:

// Input: [7,8,9,11,12]
// Output: 1

fn first_missing_positive(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    let n = nums.len() as i32;

    // first, sanitize the nums, set all unrelated numbers to n+1, thus frees up some bits we can use
    for x in nums.iter_mut() {
        if *x <= 0 || *x > n {
            *x = n + 1
        }
    }

    // next, set nums[k-1] to negative to indicate that k is found
    for i in 0..n as usize {
        let x = nums[i].abs();
        if x <= n {
            nums[x as usize - 1] = -nums[x as usize - 1].abs()
        }
    }

    nums.iter().position(|&x| x > 0).unwrap_or(n as _) as i32 + 1
}

fn main() {
    assert_eq!(first_missing_positive(vec![1,2,0]), 3);
    assert_eq!(first_missing_positive(vec![3,4,-1,1]), 2);
    assert_eq!(first_missing_positive(vec![7,8,9,11,12]), 1);
}

// faster than 100.00%, less memory than 100.00%.