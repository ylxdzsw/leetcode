// Given a sorted array and a target value, return the index if the target is found. If not, return the index where it would be if it were inserted in order.

// You may assume no duplicates in the array.

// Example 1:

// Input: [1,3,5,6], 5
// Output: 2
// Example 2:

// Input: [1,3,5,6], 2
// Output: 1
// Example 3:

// Input: [1,3,5,6], 7
// Output: 4
// Example 4:

// Input: [1,3,5,6], 0
// Output: 0

fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    match nums.binary_search(&target) {
        Ok(x) => x as _,
        Err(x) => x as _
    }
}

fn main() {
    assert_eq!(search_insert(vec![1,3,5,6], 5), 2);
    assert_eq!(search_insert(vec![1,3,5,6], 2), 1);
    assert_eq!(search_insert(vec![1,3,5,6], 7), 4);
    assert_eq!(search_insert(vec![1,3,5,6], 0), 0);
}

// faster than 100.00%, less memory than 100.00%.