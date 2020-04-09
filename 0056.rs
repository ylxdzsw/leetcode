// Given a collection of intervals, merge all overlapping intervals.

// Example 1:

// Input: [[1,3],[2,6],[8,10],[15,18]]
// Output: [[1,6],[8,10],[15,18]]
// Explanation: Since intervals [1,3] and [2,6] overlaps, merge them into [1,6].
// Example 2:

// Input: [[1,4],[4,5]]
// Output: [[1,5]]
// Explanation: Intervals [1,4] and [4,5] are considered overlapping.

fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut intervals = intervals;
    intervals.sort();

    let mut result: Vec<Vec<i32>> = vec![];
    for i in intervals {
        match result.last_mut() {
            Some(last) if i[0] <= last[1] => last[1] = std::cmp::max(last[1], i[1]),
            _ => result.push(i)
        }
    }

    result
}

fn main() {
    assert_eq!(merge(vec![vec![1,3],vec![2,6],vec![8,10],vec![15,18]]), &[&[1,6],&[8,10],&[15,18]]);
    assert_eq!(merge(vec![vec![1,4],vec![4,5]]), &[&[1,5]]);
}

// faster than 100.00%, less memory than 100.00%.