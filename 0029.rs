// Given two integers dividend and divisor, divide two integers without using multiplication, division and mod operator.

// Return the quotient after dividing dividend by divisor.

// The integer division should truncate toward zero, which means losing its fractional part. For example, truncate(8.345) = 8 and truncate(-2.7335) = -2.

// Example 1:

// Input: dividend = 10, divisor = 3
// Output: 3
// Explanation: 10/3 = truncate(3.33333..) = 3.
// Example 2:

// Input: dividend = 7, divisor = -3
// Output: -2
// Explanation: 7/-3 = truncate(-2.33333..) = -2.
// Note:

// Both dividend and divisor will be 32-bit signed integers.
// The divisor will never be 0.
// Assume we are dealing with an environment which could only store integers within the 32-bit signed integer range: [â231,  231 â 1]. For the purpose of this problem, assume that your function returns 231 â 1 when the division result overflows.

fn divide(dividend: i32, divisor: i32) -> i32 {
    use std::i32::MIN;

    // try divide and return quotient and remainnings. y > 0.
    fn _divide(x: i32, y: i32) -> (i32, i32) {
        if x != MIN && x.wrapping_abs() < y {
            return (0, x)
        }

        let (mut q, mut r) = if x != 1 && x != -1 { // x.abs() >= 2
            let (q, r) = _divide(x >> 1, y);
            (q << 1, (r << 1) + (x & 0x01))
        } else {
            (0, x)
        };

        while r == MIN || r.wrapping_abs() >= y {
            match r.signum() {
                1 => { q += 1; r -= y }
                -1 => { q -= 1; r += y }
                _ => unreachable!()
            }
        }

        (q, r)
    }

    if divisor == MIN { // special case
        if dividend == MIN {
            return 1
        } else {
            return 0
        }
    }

    let (q, _) = _divide(dividend, divisor.wrapping_abs());
    match divisor.signum() {
        1 => q,
        -1 => q.checked_neg().unwrap_or(std::i32::MAX),
        _ => unreachable!()
    }
}

fn main() {
    assert_eq!(divide(10, 3), 3);
    assert_eq!(divide(7, -3), -2);
}

// faster than 100.00%, less memory than 100.00%.