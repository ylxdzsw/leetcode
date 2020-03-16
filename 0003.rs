// Given a string, find the length of the longest substring without repeating characters.

// Example 1:

// Input: "abcabcbb"
// Output: 3 
// Explanation: The answer is "abc", with the length of 3. 
// Example 2:

// Input: "bbbbb"
// Output: 1
// Explanation: The answer is "b", with the length of 1.
// Example 3:

// Input: "pwwkew"
// Output: 3
// Explanation: The answer is "wke", with the length of 3. 
//              Note that the answer must be a substring, "pwke" is a subsequence and not a substring.


fn length_of_longest_substring(s: String) -> i32 {
    let s = s.as_bytes(); // assume ASCII
    let mut max_so_far = 0;
    let mut start = 0;

    'outer: for (end, c) in s.iter().enumerate() {
        for i in start..end {
            if c == unsafe { s.get_unchecked(i) } {
                start = i+1;
                continue 'outer // there is at most one cut point
            }
        }
        max_so_far = std::cmp::max(max_so_far, end - start + 1)
    }

    max_so_far as _
}

fn main() {
    assert_eq!(length_of_longest_substring("abcabcbb".to_string()), 3);
    assert_eq!(length_of_longest_substring("bbbbb".to_string()), 1);
    assert_eq!(length_of_longest_substring("pwwkew".to_string()), 3)
}

// faster than 100.00%, less memory than 100.00%.