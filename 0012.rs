// Roman numerals are represented by seven different symbols: I, V, X, L, C, D and M.

// Symbol       Value
// I             1
// V             5
// X             10
// L             50
// C             100
// D             500
// M             1000
// For example, two is written as II in Roman numeral, just two one's added together. Twelve is written as, XII, which is simply X + II. The number twenty seven is written as XXVII, which is XX + V + II.

// Roman numerals are usually written largest to smallest from left to right. However, the numeral for four is not IIII. Instead, the number four is written as IV. Because the one is before the five we subtract it making four. The same principle applies to the number nine, which is written as IX. There are six instances where subtraction is used:

// I can be placed before V (5) and X (10) to make 4 and 9. 
// X can be placed before L (50) and C (100) to make 40 and 90. 
// C can be placed before D (500) and M (1000) to make 400 and 900.
// Given an integer, convert it to a roman numeral. Input is guaranteed to be within the range from 1 to 3999.

// Example 1:

// Input: 3
// Output: "III"
// Example 2:

// Input: 4
// Output: "IV"
// Example 3:

// Input: 9
// Output: "IX"
// Example 4:

// Input: 58
// Output: "LVIII"
// Explanation: L = 50, V = 5, III = 3.
// Example 5:

// Input: 1994
// Output: "MCMXCIV"
// Explanation: M = 1000, CM = 900, XC = 90 and IV = 4.

fn int_to_roman(num: i32) -> String {
    let mut res = String::new();
    let mut num = num as usize;

    while num >= 1000 { res.push('M'); num -= 1000 }
    if num >= 900 { res.push_str("CM"); num -= 900 }
    if num >= 500 { res.push('D'); num -= 500 }
    if num >= 400 { res.push_str("CD"); num -= 400 }
    while num >= 100 { res.push('C'); num -= 100 }
    if num >= 90 { res.push_str("XC"); num -= 90 }
    if num >= 50 { res.push('L'); num -= 50 }
    if num >= 40 { res.push_str("XL"); num -= 40 }
    while num >= 10 { res.push('X'); num -= 10 }
    if num >= 9 { res.push_str("IX"); num -= 9 }
    if num >= 5 { res.push('V'); num -= 5 }
    if num >= 4 { res.push_str("IV"); num -= 4 }
    while num >= 1 { res.push('I'); num -= 1 }

    res
}

fn main() {
    assert_eq!(int_to_roman(3), "III");
    assert_eq!(int_to_roman(4), "IV");
    assert_eq!(int_to_roman(9), "IX");
    assert_eq!(int_to_roman(58), "LVIII");
    assert_eq!(int_to_roman(1994), "MCMXCIV")
}

// faster than 100.00%, less memory than 100.00%.