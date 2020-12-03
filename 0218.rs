// A city's skyline is the outer contour of the silhouette formed by all the buildings in that city when viewed from a distance. Given the locations and heights of all the buildings, return the skyline formed by these buildings collectively.

// The geometric information of each building is given in the array buildings where buildings[i] = [lefti, righti, heighti]:

// lefti is the x coordinate of the left edge of the ith building.
// righti is the x coordinate of the right edge of the ith building.
// heighti is the height of the ith building.
// You may assume all buildings are perfect rectangles grounded on an absolutely flat surface at height 0.

// The skyline should be represented as a list of "key points" sorted by their x-coordinate in the form [[x1,y1],[x2,y2],...]. Each key point is the left endpoint of some horizontal segment in the skyline except the last point in the list, which always has a y-coordinate  0 and is used to mark the skyline's termination where the rightmost building ends. Any ground between the leftmost and rightmost buildings should be part of the skyline's contour.

// Note: There must be no consecutive horizontal lines of equal height in the output skyline. For instance, [...,[2 3],[4 5],[7 5],[11 5],[12 7],...] is not acceptable; the three lines of height 5 should be merged into one in the final output as such: [...,[2 3],[4 5],[12 7],...]

// Example 1:

// Input: buildings = [[2,9,10],[3,7,15],[5,12,12],[15,20,10],[19,24,8]]
// Output: [[2,10],[3,15],[7,12],[12,0],[15,10],[20,8],[24,0]]

// Example 2:

// Input: buildings = [[0,2,3],[2,5,3]]
// Output: [[0,3],[5,0]]

// Constraints:

// 1 <= buildings.length <= 10^4
// 0 <= lefti < righti <= 2^31 - 1
// 1 <= heighti <= 2^31 - 1
// buildings is sorted by lefti in non-decreasing order.

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Building {
    left: i32,
    right: i32,
    height: i32
}

impl core::cmp::Ord for Building {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.left.cmp(&other.left).reverse().then(self.height.cmp(&other.height))
    }
}

impl core::cmp::PartialOrd for Building {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

use core::cmp::Ordering::{Greater, Less, Equal};

fn get_skyline(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut heap: std::collections::BinaryHeap<_> = buildings.into_iter().map(|x| Building { left: x[0], right: x[1], height: x[2]} ).collect();

    let mut c = heap.pop().unwrap();
    let mut result = Vec::with_capacity(heap.len());
    while let Some(mut b) = heap.pop() {
        match (b.left.cmp(&c.left), b.height.cmp(&c.height), b.right.cmp(&c.right), b.left.cmp(&c.right)) {
            (_, _, _, Greater) => { // a gap between
                result.push(vec![c.left, c.height]);
                result.push(vec![c.right, 0]);
                c = b
            }
            (Greater, Greater, Greater, _) | (Greater, Greater, Equal, _) => { // the right part of c is covered by b
                result.push(vec![c.left, c.height]);
                c = b
            }
            (Greater, Greater, Less, _) => { // c is cut by b in the middle
                result.push(vec![c.left, c.height]);
                c.left = b.right;
                heap.push(c);
                c = b
            }
            (Greater, Equal, _, _) => { // merge
                c.right = core::cmp::max(c.right, b.right)
            }
            (Greater, Less, Less, _) | (Greater, Less, Equal, _) => {} // b is fully covered by c
            (Greater, Less, Greater, Less) => { // left part of b is covered by c
                b.left = c.right;
                heap.push(b);
            }
            (Greater, Less, Greater, Equal) => { // adjuscent
                result.push(vec![c.left, c.height]);
                c = b;
            }
            (Equal, Less, Greater, _) => {
                b.left = c.right;
                heap.push(b)
            }
            (Equal, Less, _, _) => {} // b is fully covered by c
            (Equal, Equal, _, _) => { // same height
                c.right = core::cmp::max(c.right, b.right)
            }
            (Equal, Greater, _, _) => unreachable!(),
            (Less, _, _, _) => unreachable!(),
        }
    }

    result.push(vec![c.left, c.height]);
    result.push(vec![c.right, 0]);

    result
}

fn main() {
    assert_eq!(get_skyline(vec![
        vec![2,9,10],
        vec![3,7,15],
        vec![5,12,12],
        vec![15,20,10],
        vec![19,24,8]
    ]), [[2,10],[3,15],[7,12],[12,0],[15,10],[20,8],[24,0]]);
    assert_eq!(get_skyline(vec![
        vec![0,2,3],
        vec![2,5,3],
    ]), [[0,3],[5,0]])
}

// faster than 88.46%, less memory than 88.46%.