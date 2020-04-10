// Given n non-negative integers representing the histogram's bar height where the width of each bar is 1, find the area of largest rectangle in the histogram.

// Above is a histogram where width of each bar is 1, given height = [2,1,5,6,2,3].

// The largest rectangle is shown in the shaded area, which has area = 10 unit.

// Example:

// Input: [2,1,5,6,2,3]
// Output: 10

fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
    fn _largest_rectangle_area(heights: &[i32]) -> i32 {
        match heights.len() {
            0 => 0,
            1 => heights[0],
            len => {
                let (s, min_height) = heights.iter().enumerate().min_by_key(|(_, &min)| min).unwrap();
                *[
                    min_height * len as i32,
                    _largest_rectangle_area(&heights[..s]),
                    _largest_rectangle_area(&heights[s+1..])
                ].iter().max().unwrap()
            }
        }
    }

    _largest_rectangle_area(&heights)
}

fn main() {
    assert_eq!(largest_rectangle_area(vec![2,1,5,6,2,3]), 10)
}

// faster than 23.08%, less memory than 100.00%.