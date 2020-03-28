// Given a string containing just the characters '(' and ')', find the length of the longest valid (well-formed) parentheses substring.

// Example 1:

// Input: "(()"
// Output: 2
// Explanation: The longest valid parentheses substring is "()"
// Example 2:

// Input: ")()())"
// Output: 4
// Explanation: The longest valid parentheses substring is "()()"
#[derive(Eq, PartialEq, Copy, Clone)]
enum Kind { Left, Right, Both }
struct Token { pub kind: Kind, pub len: usize }

// repeatedly merge tokens by the two rules and sum the len as the new token
// Left + Right = Both
// Left + Both = Left
// Both + Both = Both
// when no tokens can be merged, calculate maximum length in the tokens
fn longest_valid_parentheses(s: String) -> i32 {
    let mut tokens: Vec<_> = s.chars().map(|c| match c {
        '(' => Token { kind: Kind::Left, len: 1 },
        ')' => Token { kind: Kind::Right, len: 1 },
        _ => unreachable!()
    }).collect();

    let mut i = 0;
    while i + 1 < tokens.len() {
        match (tokens[i].kind, tokens[i+1].kind) {
            (Kind::Left, Kind::Both) => {
                tokens[i].len += tokens[i+1].len;
                tokens.remove(i+1);
            }
            (Kind::Left, Kind::Right) => {
                tokens[i].kind = Kind::Both;
                tokens[i].len += tokens[i+1].len;
                tokens.remove(i+1);
                if i > 0 { // revisit last one since we changed the kind of i
                    i -= 1
                }
            }
            (Kind::Both, Kind::Both) => {
                tokens[i].len += tokens[i+1].len;
                tokens.remove(i+1);
            }
            _ => i += 1
        }
    }

    tokens.iter().map(|token| match token.kind {
        Kind::Left | Kind::Right => token.len - 1,
        Kind::Both => token.len
    }).max().map(|x| x as _).unwrap_or(0)
}

fn main() {
    assert_eq!(longest_valid_parentheses("(()".to_string()), 2);
    assert_eq!(longest_valid_parentheses(")()())".to_string()), 4);
}

// faster than 50.00%, less memory than 100.00%.
// TODO: need to use an actual doubly linked list (std linked_list_cursors is not stable)