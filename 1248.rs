// Given an array of integers nums and an integer k. A subarray is called nice if there are k odd numbers on it.

// Return the number of nice sub-arrays.

// Example 1:

// Input: nums = [1,1,2,1,1], k = 3
// Output: 2
// Explanation: The only sub-arrays with 3 odd numbers are [1,1,2,1] and [1,2,1,1].
// Example 2:

// Input: nums = [2,4,6], k = 1
// Output: 0
// Explanation: There is no odd numbers in the array.
// Example 3:

// Input: nums = [2,2,2,1,2,2,1,2,2,2], k = 2
// Output: 16

// Constraints:

// 1 <= nums.length <= 50000
// 1 <= nums[i] <= 10^5
// 1 <= k <= nums.length

fn number_of_subarrays(nums: Vec<i32>, k: i32) -> i32 {
    // build an array of gap lengths (gap are continous even numbers). We may reuse `nums` to get O(1) extra space.
    let mut gaps = Vec::with_capacity(nums.len());
    gaps.push(1);
    for i in nums {
        if i % 2 == 0 {
            *gaps.last_mut().unwrap() += 1
        } else {
            gaps.push(1)
        }
    }

    let mut sum = 0;
    for i in 0..gaps.len() as i32 - k {
        sum += gaps[i as usize] * gaps[i as usize + k as usize]
    }

    sum
}

fn main() {
    assert_eq!(number_of_subarrays(vec![1,1,2,1,1], 3), 2);
    assert_eq!(number_of_subarrays(vec![2,4,6], 1), 0);
    assert_eq!(number_of_subarrays(vec![2,2,2,1,2,2,1,2,2,2], 2), 16);
}

// faster than 100.00%, less memory than 100.00%.
// with_capacity is necessary for 100.00% time. Need to reuse the `nums` array or even hackier calculating gaps on the fly for further optimization.