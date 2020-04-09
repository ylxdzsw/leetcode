// Given a set of non-overlapping intervals, insert a new interval into the intervals (merge if necessary).

// You may assume that the intervals were initially sorted according to their start times.

// Example 1:

// Input: intervals = [[1,3],[6,9]], newInterval = [2,5]
// Output: [[1,5],[6,9]]
// Example 2:

// Input: intervals = [[1,2],[3,5],[6,7],[8,10],[12,16]], newInterval = [4,8]
// Output: [[1,2],[3,10],[12,16]]
// Explanation: Because the new interval [4,8] overlaps with [3,5],[6,7],[8,10].
// NOTE: input types have been changed on April 15, 2019. Please reset to default code definition to get new method signature.

fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
    let mut intervals = intervals;
    intervals.push(new_interval);
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
    assert_eq!(insert(vec![vec![1,3],vec![6,9]], vec![2,5]), &[&[1,5],&[6,9]]);
    assert_eq!(insert(vec![vec![1,2],vec![3,5],vec![6,7],vec![8,10],vec![12,16]], vec![4,8]), &[&[1,2],&[3,10],&[12,16]]);
}

// faster than 100.00%, less memory than 100.00%.