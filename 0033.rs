// Suppose an array sorted in ascending order is rotated at some pivot unknown to you beforehand.

// (i.e., [0,1,2,4,5,6,7] might become [4,5,6,7,0,1,2]).

// You are given a target value to search. If found in the array return its index, otherwise return -1.

// You may assume no duplicate exists in the array.

// Your algorithm's runtime complexity must be in the order of O(log n).

// Example 1:

// Input: nums = [4,5,6,7,0,1,2], target = 0
// Output: 4
// Example 2:

// Input: nums = [4,5,6,7,0,1,2], target = 3
// Output: -1

#[allow(clippy::or_fun_call)]
fn search(nums: Vec<i32>, target: i32) -> i32 {
    fn _search(nums: &[i32], target: i32) -> Option<usize> {
        let l = nums.len();
        match l {
            0 => None,
            1 => if nums[0] == target { Some(0) } else { None }
            2 => _search(&nums[..1], target).or(_search(&nums[1..], target).map(|x| x + 1)),
            _ => {
                let m = l >> 1;
                if nums[m] > nums[0] && nums[m] > nums[l-1] { // the wrap point is on the right
                    if nums[m] > target {
                        if let Ok(i) = nums[..m].binary_search(&target) {
                            return Some(i)
                        }
                    }
                    _search(&nums[m..], target).map(|i| i + m)
                } else if nums[m] < nums[0] && nums[m] < nums[l-1] { // the wrap point is on the left
                    if nums[m] <= target {
                        if let Ok(i) = nums[m..].binary_search(&target) {
                            return Some(i + m)
                        }
                    }
                    _search(&nums[..m], target).map(|i| i)
                } else { // there is no wrap point
                    nums.binary_search(&target).ok()
                }
            }
        }
    }

    _search(&nums[..], target).map(|x| x as _).unwrap_or(-1)
}

fn main() {
    assert_eq!(search(vec![4,5,6,7,0,1,2], 0), 4);
    assert_eq!(search(vec![4,5,6,7,0,1,2], 3), -1);
}

// faster than 100.00%, less memory than 100.00%.