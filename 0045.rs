// Given an array of non-negative integers, you are initially positioned at the first index of the array.

// Each element in the array represents your maximum jump length at that position.

// Your goal is to reach the last index in the minimum number of jumps.

// Example:

// Input: [2,3,1,1,4]
// Output: 2
// Explanation: The minimum number of jumps to reach the last index is 2.
//     Jump 1 step from index 0 to 1, then 3 steps to the last index.

fn jump(nums: Vec<i32>) -> i32 {
    // left and right (inclusive) indicate the range that can be reached by [step] steps but can't be reached by [step - 1] steps
    let mut left = 0;
    let mut right = 0;
    let mut step = 0;

    while right < nums.len() - 1 { // if the destination can't be reached in [step] steps
        step += 1;
        let mut farest = right;
        for i in left..=right {
            if i + nums[i] as usize > farest {
                farest = i + nums[i] as usize
            }
        }
        left = right + 1;
        right = farest
    }

    step
}

fn main() {
    assert_eq!(jump(vec![2,3,1,1,4]), 2);
}

// faster than 100.00%, less memory than 100.00%.