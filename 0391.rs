// Given an array rectangles where rectangles[i] = [xi, yi, ai, bi] represents an axis-aligned rectangle. The bottom-left point of the rectangle is (xi, yi) and the top-right point of it is (ai, bi).

// Return true if all the rectangles together form an exact cover of a rectangular region.

// Example 1:

// Input: rectangles = [[1,1,3,3],[3,1,4,2],[3,2,4,4],[1,3,2,4],[2,3,3,4]]
// Output: true
// Explanation: All 5 rectangles together form an exact cover of a rectangular region.

// Example 2:

// Input: rectangles = [[1,1,2,3],[1,3,2,4],[3,1,4,2],[3,2,4,4]]
// Output: false
// Explanation: Because there is a gap between the two rectangular regions.

// Example 3:

// Input: rectangles = [[1,1,3,3],[3,1,4,2],[1,3,2,4],[2,2,4,4]]
// Output: false
// Explanation: Because two of the rectangles overlap with each other.
 
// Constraints:

// 1 <= rectangles.length <= 2 * 10^4
// rectangles[i].length == 4
// -10^5 <= xi, yi, ai, bi <= 10^5

fn is_rectangle_cover(rectangles: Vec<Vec<i32>>) -> bool {
    use core::cmp::{min, max};

    fn bounded_area(rectangle: &[i32; 4], bounding_box: &[i32; 4]) -> u32 {
        let &[xb, yb, ab, bb] = bounding_box;
        let &[x, y, a, b] = rectangle;
        let h = min(b, bb) - max(y, yb);
        let w = min(a, ab) - max(x, xb);
        if h <= 0 || w <= 0 {
            return 0
        }
        (h * w) as _
    }

    // idea: recursively split the whole area and see if the two parts are both perfect rectangles
    fn _is_rectangle_cover(rectangles: &[[i32; 4]], bounding_box: [i32; 4]) -> bool {
        // 1. if the areas do not match, it must not be a perfect rectangle
        let bounding_area = bounded_area(&bounding_box, &bounding_box);
        let total_area: u32 = rectangles.iter().map(|rec| bounded_area(rec, &bounding_box)).sum();
        if total_area != bounding_area {
            return false
        }

        // 2. if the areas do match and there is only one rectangle, it is the perfect rectangle
        let n_rects = rectangles.iter().filter(|rec| bounded_area(rec, &bounding_box) > 0).count();
        if n_rects == 1 {
            return true
        }

        // 3. otherwise, split the box and check the two parts
        let [xb, yb, ab, bb] = bounding_box;
        for &[x, y, a, b] in rectangles {
            if x > xb && x < ab {
                return _is_rectangle_cover(rectangles, [x, yb, ab, bb]) && _is_rectangle_cover(rectangles, [xb, yb, x, bb])
            }
            if a > xb && a < ab {
                return _is_rectangle_cover(rectangles, [a, yb, ab, bb]) && _is_rectangle_cover(rectangles, [xb, yb, a, bb])
            }
            if y > yb && y < bb {
                return _is_rectangle_cover(rectangles, [xb, y, ab, bb]) && _is_rectangle_cover(rectangles, [xb, yb, ab, y])
            }
            if b > yb && b < bb {
                return _is_rectangle_cover(rectangles, [xb, b, ab, bb]) && _is_rectangle_cover(rectangles, [xb, yb, ab, b])
            }
        }

        unreachable!()
    }

    let rectangles: Vec<[i32; 4]> = rectangles.iter().map(|x| x.clone().try_into().unwrap()).collect();
    let xb = rectangles.iter().map(|&[x, _y, _a, _b]| x).min().unwrap();
    let yb = rectangles.iter().map(|&[_x, y, _a, _b]| y).min().unwrap();
    let ab = rectangles.iter().map(|&[_x, _y, a, _b]| a).max().unwrap();
    let bb = rectangles.iter().map(|&[_x, _y, _a, b]| b).max().unwrap();
    _is_rectangle_cover(&rectangles, [xb, yb, ab, bb])
}

fn main() {
    assert!(is_rectangle_cover(vec![vec![1,1,3,3],vec![3,1,4,2],vec![3,2,4,4],vec![1,3,2,4],vec![2,3,3,4]]));
    assert!(!is_rectangle_cover(vec![vec![1,1,2,3],vec![1,3,2,4],vec![3,1,4,2],vec![3,2,4,4]]));
    assert!(!is_rectangle_cover(vec![vec![1,1,3,3],vec![3,1,4,2],vec![1,3,2,4],vec![2,2,4,4]]));
}

// Time Limit Exceeded. Need to improve.