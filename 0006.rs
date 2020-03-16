// The string "PAYPALISHIRING" is written in a zigzag pattern on a given number of rows like this: (you may want to display this pattern in a fixed font for better legibility)

// P   A   H   N
// A P L S I I G
// Y   I   R
// And then read line by line: "PAHNAPLSIIGYIR"

// Write the code that will take a string and make this conversion given a number of rows:

// string convert(string s, int numRows);
// Example 1:

// Input: s = "PAYPALISHIRING", numRows = 3
// Output: "PAHNAPLSIIGYIR"
// Example 2:

// Input: s = "PAYPALISHIRING", numRows = 4
// Output: "PINALSIGYAHRPI"
// Explanation:

// P     I    N
// A   L S  I G
// Y A   H R
// P     I

fn convert(s: String, num_rows: i32) -> String {
    if num_rows == 1 {
        return s
    }

    let s = s.as_bytes(); // assume ASCII
    let num_rows = num_rows as usize;
    let mut result = Vec::with_capacity(s.len());

    let num_middle = num_rows - 2;

    // the first line
    let mut i = 0;
    while i < s.len() {
        result.push(s[i]);
        i += 2 * num_middle + 2
    }

    // middle lines
    for l in 0..num_middle {
        let mut i = 1 + l;
        while i < s.len() {
            result.push(s[i]);
            let j = i + 2 * (num_middle - l - 1) + 2;
            if j < s.len() {
                result.push(s[j])
            }
            i += 2 * num_middle + 2
        }
    }
    
    // and the last line
    let mut i = num_rows - 1;
    while i < s.len() {
        result.push(s[i]);
        i += 2 * num_middle + 2
    }

    unsafe { String::from_utf8_unchecked(result) }
}

fn main() {
    assert_eq!(convert("PAYPALISHIRING".to_string(), 3), "PAHNAPLSIIGYIR");
    assert_eq!(convert("PAYPALISHIRING".to_string(), 4), "PINALSIGYAHRPI");
}

// faster than 100.00% (with bounds check on), less memory than 100.00%.