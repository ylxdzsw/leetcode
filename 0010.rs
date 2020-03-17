// Given an input string (s) and a pattern (p), implement regular expression matching with support for '.' and '*'.

// '.' Matches any single character.
// '*' Matches zero or more of the preceding element.
// The matching should cover the entire input string (not partial).

// Note:

// s could be empty and contains only lowercase letters a-z.
// p could be empty and contains only lowercase letters a-z, and characters like . or *.
// Example 1:

// Input:
// s = "aa"
// p = "a"
// Output: false
// Explanation: "a" does not match the entire string "aa".
// Example 2:

// Input:
// s = "aa"
// p = "a*"
// Output: true
// Explanation: '*' means zero or more of the preceding element, 'a'. Therefore, by repeating 'a' once, it becomes "aa".
// Example 3:

// Input:
// s = "ab"
// p = ".*"
// Output: true
// Explanation: ".*" means "zero or more (*) of any character (.)".
// Example 4:

// Input:
// s = "aab"
// p = "c*a*b"
// Output: true
// Explanation: c can be repeated 0 times, a can be repeated 1 time. Therefore, it matches "aab".
// Example 5:

// Input:
// s = "mississippi"
// p = "mis*is*p*."
// Output: false

fn is_match(s: String, p: String) -> bool {
    let s: Vec<_> = s.chars().collect();
    let p: Vec<_> = p.chars().collect();

    fn _is_match(s: &[char], p: &[char]) -> bool {
        if p.is_empty() {
            return s.is_empty()
        }

        if p.len() >= 2 && p[1] == '*' {
            if _is_match(s, &p[2..]) {
                return true
            }
            !s.is_empty() && (s[0] == p[0] || p[0] == '.') && (_is_match(&s[1..], p) || _is_match(&s[1..], &p[2..]))
        } else {
            !s.is_empty() && (s[0] == p[0] || p[0] == '.') && _is_match(&s[1..], &p[1..])
        }
    }

    _is_match(&s, &p)
}

fn main() {
    assert!(!is_match("aa".to_string(), "a".to_string()));
    assert!(is_match("aa".to_string(), "a*".to_string()));
    assert!(is_match("ab".to_string(), ".*".to_string()));
    assert!(is_match("aab".to_string(), "c*a*b".to_string()));
    assert!(!is_match("mississippi".to_string(), "mis*is*p*".to_string()));
}

// faster than 9.09%, less memory than 100.00%.