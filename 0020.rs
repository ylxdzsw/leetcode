// Given a string containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string is valid.

// An input string is valid if:

// Open brackets must be closed by the same type of brackets.
// Open brackets must be closed in the correct order.
// Note that an empty string is also considered valid.

// Example 1:

// Input: "()"
// Output: true
// Example 2:

// Input: "()[]{}"
// Output: true
// Example 3:

// Input: "(]"
// Output: false
// Example 4:

// Input: "([)]"
// Output: false
// Example 5:

// Input: "{[]}"
// Output: true

fn is_valid(s: String) -> bool {
    fn tag_map(c: char) -> Option<char> {
        match c {
            '(' => Some(')'),
            '[' => Some(']'),
            '{' => Some('}'),
            _ => None
        }
    }

    fn _parse(s: &[char], close: Option<char>) -> Option<usize> {
        let mut i = 0;
        while let Some(c) = s.get(i).copied().and_then(tag_map) {
            i += _parse(&s[i+1..], Some(c))? + 1
        }
        if s.get(i).copied() == close {
            Some(i+1)
        } else {
            None
        }
    }

    let s: Vec<_> = s.chars().collect();
    _parse(&s, None).is_some()
}

fn main() {
    assert!(is_valid("()".to_string()));
    assert!(is_valid("()[]{}".to_string()));
    assert!(!is_valid("(]".to_string()));
    assert!(!is_valid("([)]".to_string()));
    assert!(is_valid("{[]}".to_string()));
}

// faster than 100.00%, less memory than 100.00%.