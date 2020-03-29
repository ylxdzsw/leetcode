// Given an array of non-negative integers, you are initially positioned at the first index of the array.

// Each element in the array represents your maximum jump length at that position.

// Determine if you are able to reach the last index.

// Example 1:

// Input: [2,3,1,1,4]
// Output: true
// Explanation: Jump 1 step from index 0 to 1, then 3 steps to the last index.
// Example 2:

// Input: [3,2,1,0,4]
// Output: false
// Explanation: You will always arrive at index 3 no matter what. Its maximum
//              jump length is 0, which makes it impossible to reach the last index.

fn can_jump(nums: Vec<i32>) -> bool {
    let mut farest = 0; // the farest reachable (inclusive) position

    for i in 0..nums.len() {
        if i > farest {
            return false
        }

        if i + nums[i] as usize > farest {
            farest = i + nums[i] as usize
        }
    }

    true
}

fn main() {
    assert!(can_jump(vec![2,3,1,1,4]));
    assert!(!can_jump(vec![3,2,1,0,4]));
}

// faster than 100.00%, less memory than 100.00%.