// Given a 32-bit signed integer, reverse digits of an integer.

// Example 1:

// Input: 123
// Output: 321
// Example 2:

// Input: -123
// Output: -321
// Example 3:

// Input: 120
// Output: 21
// Note:
// Assume we are dealing with an environment which could only store integers within the 32-bit signed integer range: [-2^31, 2^31-1]. For the purpose of this problem, assume that your function returns 0 when the reversed integer overflows.

fn reverse(x: i32) -> i32 {
    let mut y = x.wrapping_abs(); // -2^31 accidently become 0 so it is not a problem
    let mut result: i32 = 0;
    
    while y != 0 {
        if let Some(new_result) = result.checked_mul(10).and_then(|x| x.checked_add(y % 10)) {
            result = new_result;
            y /= 10
        } else {
            return 0
        }
    }
    
    result * x.signum()
}

fn main() {
    assert_eq!(reverse(123), 321);
    assert_eq!(reverse(-123), -321);
    assert_eq!(reverse(120), 21);
}

// faster than 100.00%, less memory than 100.00%.