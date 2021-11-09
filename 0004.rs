// Given two sorted arrays nums1 and nums2 of size m and n respectively, return the median of the two sorted arrays.

// The overall run time complexity should be O(log (m+n)).

// Example 1:

// Input: nums1 = [1,3], nums2 = [2]
// Output: 2.00000
// Explanation: merged array = [1,2,3] and median is 2.
// Example 2:

// Input: nums1 = [1,2], nums2 = [3,4]
// Output: 2.50000
// Explanation: merged array = [1,2,3,4] and median is (2 + 3) / 2 = 2.5.
// Example 3:

// Input: nums1 = [0,0], nums2 = [0,0]
// Output: 0.00000
// Example 4:

// Input: nums1 = [], nums2 = [1]
// Output: 1.00000
// Example 5:

// Input: nums1 = [2], nums2 = []
// Output: 2.00000

// Constraints:

// nums1.length == m
// nums2.length == n
// 0 <= m <= 1000
// 0 <= n <= 1000
// 1 <= m + n <= 2000
// -10^6 <= nums1[i], nums2[i] <= 10^6

fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    fn _find_median_sorted_arrays<'a>(mut nums1: &'a [i32], mut nums2: &'a [i32]) -> f64 {
        if nums1.len() < nums2.len() { // ensure nums1 is no shorter than nums2 to simplify following code
            std::mem::swap(&mut nums1, &mut nums2)
        }

        match (nums1.len(), nums2.len()) {
            (1, 0) => return nums1[0] as _,
            (1, 1) => return ((nums1[0] + nums2[0]) as f64) / 2.,
            (2, 0) => return ((nums1[0] + nums1[1]) as f64) / 2.,
            (l, 0) => if l % 2 == 0 {
                return ((nums1[l/2-1] + nums1[l/2]) as f64) / 2.
            } else {
                return nums1[l/2]
            }
            (l1, l2) => {
                todo!() // discard at least half of the elements?
            }
        }
    }

    _find_median_sorted_arrays(&nums1[..], &nums2[..])
}

fn main() {
    assert_eq!(find_median_sorted_arrays(vec![1,3], vec![2]), 2.);
    assert_eq!(find_median_sorted_arrays(vec![1,2], vec![3,4]), 2.5);
    assert_eq!(find_median_sorted_arrays(vec![0,0], vec![0,0]), 0.);
    assert_eq!(find_median_sorted_arrays(vec![], vec![1]), 1.);
    assert_eq!(find_median_sorted_arrays(vec![2], vec![]), 2.);
}
