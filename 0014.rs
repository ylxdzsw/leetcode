// Write a function to find the longest common prefix string amongst an array of strings.

// If there is no common prefix, return an empty string "".

// Example 1:

// Input: ["flower","flow","flight"]
// Output: "fl"
// Example 2:

// Input: ["dog","racecar","car"]
// Output: ""
// Explanation: There is no common prefix among the input strings.
// Note:

// All given inputs are in lowercase letters a-z.

fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.is_empty() {
        return "".to_string()
    }

    let strs: Vec<_> = strs.iter().map(|x| x.as_bytes()).collect();
    let mut result = vec![];

    'ret: for i in 0.. {
        if i >= strs[0].len() {
            break 'ret
        }
        let c = strs[0][i];
        for s in &strs[1..] {
            if i >= s.len() || c != s[i] {
                break 'ret
            }
        }
        result.push(c)
    }

    unsafe { String::from_utf8_unchecked(result) }
}

fn main() {
    assert_eq!(longest_common_prefix(vec![
        "flower".to_string(),
        "flow".to_string(),
        "flight".to_string(),
    ]), "fl");
    assert_eq!(longest_common_prefix(vec![
        "dog".to_string(),
        "racecar".to_string(),
        "car".to_string(),
    ]), "");
}

// faster than 100.00%, less memory than 100.00%.