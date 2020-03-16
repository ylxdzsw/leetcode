// Given a string s, find the longest palindromic substring in s. You may assume that the maximum length of s is 1000.

// Example 1:

// Input: "babad"
// Output: "bab"
// Note: "aba" is also a valid answer.
// Example 2:

// Input: "cbbd"
// Output: "bb"

fn longest_palindrome(s: String) -> String {
    let s = s.as_bytes(); // assume ASCII

    let mut longest_so_far = &s[..0];

    fn expand_palindrome(s: &[u8], start: usize, end: usize) -> &[u8] {
        if start >= 1 && end <= s.len()-2 && s[start-1] == s[end+1] {
            expand_palindrome(s, start - 1, end + 1)
        } else {
            &s[start..=end]
        }
    }

    for i in 0..s.len() {
        let x = expand_palindrome(s, i, i);
        if x.len() > longest_so_far.len() {
            longest_so_far = x
        }

        if i+1 < s.len() && s[i] == s[i+1] {
            let x = expand_palindrome(s, i, i + 1);
            if x.len() > longest_so_far.len() {
                longest_so_far = x
            }
        }
    }

    unsafe { String::from_utf8_unchecked(longest_so_far.to_vec()) }
}

fn main() {
    assert!(longest_palindrome("babad".to_string()) == "bab" ||
            longest_palindrome("babad".to_string()) == "aba");
    assert!(longest_palindrome("cbbd".to_string()) == "bb")
}

// faster than 47.27%, less memory than 100.00%.