// Given n non-negative integers a1, a2, ..., an , where each represents a point at coordinate (i, ai). n vertical lines are drawn such that the two endpoints of line i is at (i, ai) and (i, 0). Find two lines, which together with x-axis forms a container, such that the container contains the most water.

// Note: You may not slant the container and n is at least 2.

// Example:

// Input: [1,8,6,2,5,4,8,3,7]
// Output: 49

fn max_area(height: Vec<i32>) -> i32 {
    let heights = &height[..];

    fn size(heights: &[i32], lb: usize, rb: usize) -> i32 {
        (rb-lb) as i32 * std::cmp::min(heights[lb], heights[rb])
    }

    #[allow(clippy::comparison_chain)]
    fn _max_area(heights: &[i32], mut best_rb: usize) -> i32 {
        if best_rb < 1 {
            return -1
        } else if best_rb == 1 {
            return size(heights, 0, 1)
        }

        // if we use the left most element as left bound
        let mut rb = best_rb; // elements larger than the best right bound can't be better since they are lower than the previous best left bounds, so moving left bound won't change the water height, but reduce water width, so they are definitly lower than best.
        while rb > 1 && heights[rb] < heights[0] { // try move right bound near, but stop when it is higher than left bound since moving further won't increase the height, but reduce the width
            rb -= 1;
            if size(heights, 0, rb) > size(heights, 0, best_rb) {
                best_rb = rb
            }
        }

        // `best_rb` now is the real best if we keep use the leftmost element as left bound. We compare it with do not using the leftmost element as left bound.
        let water = _max_area(&heights[1..], best_rb - 1); // adjust best index relative to new heights
        std::cmp::max(size(heights, 0, best_rb), water)
    }

    _max_area(heights, heights.len() - 1)
}

fn main() {
    assert_eq!(max_area(vec![1,8,6,2,5,4,8,3,7]), 49);
}

// faster than 41.38%, less memory than 100.00%.
// TODO: somewhat messy. The two-pointer approch in LeetCode solution is much more clear.