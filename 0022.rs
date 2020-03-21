// Given n pairs of parentheses, write a function to generate all combinations of well-formed parentheses.

// For example, given n = 3, a solution set is:

// [
//   "((()))",
//   "(()())",
//   "(())()",
//   "()(())",
//   "()()()"
// ]

fn generate_parenthesis(n: i32) -> Vec<String> {
    if n == 0 {
        return vec!["".to_string()]
    }

    let mut result = vec![];
    for i in 0..n {
        for inner in generate_parenthesis(i) {
            for outer in generate_parenthesis(n-i-1) {
                result.push(format!("({}){}", inner, outer))
            }
        }
    }

    result
}

fn main() {
    let mut a = generate_parenthesis(3);
    a.sort_unstable();
    let mut b: Vec<_> = [
        "((()))",
        "(()())",
        "(())()",
        "()(())",
        "()()()"
    ].iter().map(|&x| x.to_string()).collect();
    b.sort_unstable();
    assert_eq!(a, b);
}

// faster than 100.00%, less memory than 100.00%.