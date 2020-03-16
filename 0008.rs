// Implement atoi which converts a string to an integer.

// The function first discards as many whitespace characters as necessary until the first non-whitespace character is found. Then, starting from this character, takes an optional initial plus or minus sign followed by as many numerical digits as possible, and interprets them as a numerical value.

// The string can contain additional characters after those that form the integral number, which are ignored and have no effect on the behavior of this function.

// If the first sequence of non-whitespace characters in str is not a valid integral number, or if no such sequence exists because either str is empty or it contains only whitespace characters, no conversion is performed.

// If no valid conversion could be performed, a zero value is returned.

// Note:

// Only the space character ' ' is considered as whitespace character.
// Assume we are dealing with an environment which could only store integers within the 32-bit signed integer range: [â231,  231 â 1]. If the numerical value is out of the range of representable values, INT_MAX (231 â 1) or INT_MIN (â231) is returned.
// Example 1:

// Input: "42"
// Output: 42
// Example 2:

// Input: "   -42"
// Output: -42
// Explanation: The first non-whitespace character is '-', which is the minus sign.
//              Then take as many numerical digits as possible, which gets 42.
// Example 3:

// Input: "4193 with words"
// Output: 4193
// Explanation: Conversion stops at digit '3' as the next character is not a numerical digit.
// Example 4:

// Input: "words and 987"
// Output: 0
// Explanation: The first non-whitespace character is 'w', which is not a numerical 
//              digit or a +/- sign. Therefore no valid conversion could be performed.
// Example 5:

// Input: "-91283472332"
// Output: -2147483648
// Explanation: The number "-91283472332" is out of the range of a 32-bit signed integer.
//              Thefore INT_MIN (-2^31) is returned.

fn my_atoi(str: String) -> i32 {
    let mut result: i32 = 0;
    let mut sign = 0;

    for c in str.chars() {
        match c {
            ' ' if sign == 0 => continue,
            '+' if sign == 0 => sign = 1,
            '-' if sign == 0 => sign = -1,
            '0'..='9' => {
                let n = c.to_digit(10).unwrap();
                if let Some(new_result) = result.checked_mul(10).and_then(|x| x.checked_add(n as _)) {
                    result = new_result;
                    if sign == 0 {
                        sign = 1
                    }
                } else {
                    return match sign {
                        1 => std::i32::MAX,
                        -1 => std::i32::MIN,
                        _ => 0
                    }
                }
            },
            _ => break
        }
    }
    
    result * sign
}

fn main() {
    assert_eq!(my_atoi("42".to_string()), 42);
    assert_eq!(my_atoi("   -42".to_string()), -42);
    assert_eq!(my_atoi("4193 with words".to_string()), 4193);
    assert_eq!(my_atoi("words and 987".to_string()), 0);
    assert_eq!(my_atoi("-91283472332".to_string()), std::i32::MIN);
}

// faster than 100.00%, less memory than 100.00%.