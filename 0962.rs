// Given an array A of integers, a ramp is a tuple (i, j) for which i < j and A[i] <= A[j].  The width of such a ramp is j - i.

// Find the maximum width of a ramp in A.  If one doesn't exist, return 0.

// Example 1:

// Input: [6,0,8,2,1,5]
// Output: 4
// Explanation: 
// The maximum width ramp is achieved at (i, j) = (1, 5): A[1] = 0 and A[5] = 5.
// Example 2:

// Input: [9,8,1,0,1,9,4,0,4,1]
// Output: 7
// Explanation: 
// The maximum width ramp is achieved at (i, j) = (2, 9): A[2] = 1 and A[9] = 1.

// Note:

// 2 <= A.length <= 50000
// 0 <= A[i] <= 50000

fn max_width_ramp(a: Vec<i32>) -> i32 {
    let mut stack = vec![]; // stack stores the indexes x such that a[x] > a[i] for any i > x
    for (i, &v) in a.iter().enumerate().rev() {
        if stack.is_empty() || v > a[*stack.last().unwrap()] {
            stack.push(i)
        }
    }

    let mut best: usize = 0;
    for (i, &v) in a.iter().enumerate() {
        while !stack.is_empty() && v <= a[*stack.last().unwrap()] {
            best = std::cmp::max(best, stack.pop().unwrap() - i);
        }
    }

    best as _
}

fn main() {
    assert_eq!(max_width_ramp(vec![6,0,8,2,1,5]), 4);
    assert_eq!(max_width_ramp(vec![9,8,1,0,1,9,4,0,4,1]), 7);
}

// faster than 100.00%, less memory than 100.00%.