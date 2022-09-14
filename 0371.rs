// Given two integers a and b, return the sum of the two integers without using the operators + and -.

// Example 1:

// Input: a = 1, b = 2
// Output: 3
// Example 2:

// Input: a = 2, b = 3
// Output: 5

// Constraints:

// -1000 <= a, b <= 1000

fn get_sum(a: i32, b: i32) -> i32 {
    fn _get_sum(a: i32, b: i32) -> i32 {
        if a == 0 || b == 0 {
            return a | b;
        }
        let sum = a ^ b;
        let carry = (a & b) << 1;
        _get_sum(sum, carry)
    }
    _get_sum(a, b)
}

fn main() {
    assert_eq!(get_sum(1, 2), 3);
    assert_eq!(get_sum(2, 3), 5);
}

// faster than 100%, less memory than 53.06%.