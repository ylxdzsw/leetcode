// The set [1, 2, 3, ..., n] contains a total of n! unique permutations.

// By listing and labeling all of the permutations in order, we get the following sequence for n = 3:

// "123"
// "132"
// "213"
// "231"
// "312"
// "321"
// Given n and k, return the kth permutation sequence.

// Example 1:

// Input: n = 3, k = 3
// Output: "213"
// Example 2:

// Input: n = 4, k = 9
// Output: "2314"
// Example 3:

// Input: n = 3, k = 1
// Output: "123"

// Constraints:

// 1 <= n <= 9
// 1 <= k <= n!

fn get_permutation(n: i32, k: i32) -> String {
    fn _get_permutation(n: usize, mut k: usize) -> Vec<u8> {
        if n == 1 {
            return vec![1] // with capacity n?
        }

        let f = (2..=n-1).into_iter().fold(1, |x, y| x * y); // factorial, representing the number of permutations of n-1 digits
        let initial = k / f + 1;
        k = k % f;

        let mut result = _get_permutation(n-1, k);
        for x in &mut result {
            if *x >= initial as _ {
                *x += 1
            }
        }
        result.push(initial as _); // add to the end for better performance. Need to reverse to get final result.
        result
    }
    
    let mut buf = _get_permutation(n as _, (k - 1) as _);
    for x in &mut buf {
        *x += '0' as u8
    }
    buf.reverse();
    String::from_utf8(buf).unwrap()
}

fn main() {
    assert_eq!(get_permutation(3, 3), "213");
    assert_eq!(get_permutation(4, 9), "2314");
    assert_eq!(get_permutation(3, 1), "123")
}

// faster than 100.00%, less memory than 72.73%.