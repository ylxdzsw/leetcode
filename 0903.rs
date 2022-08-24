// You are given a string s of length n where s[i] is either:

// 'D' means decreasing, or
// 'I' means increasing.
// A permutation perm of n + 1 integers of all the integers in the range [0, n] is called a valid permutation if for all valid i:

// If s[i] == 'D', then perm[i] > perm[i + 1], and
// If s[i] == 'I', then perm[i] < perm[i + 1].
// Return the number of valid permutations perm. Since the answer may be large, return it modulo 10^9 + 7.

// Example 1:

// Input: s = "DID"
// Output: 5
// Explanation: The 5 valid permutations of (0, 1, 2, 3) are:
// (1, 0, 3, 2)
// (2, 0, 3, 1)
// (2, 1, 3, 0)
// (3, 0, 2, 1)
// (3, 1, 2, 0)
// Example 2:

// Input: s = "D"
// Output: 1

// Constraints:

// n == s.length
// 1 <= n <= 200
// s[i] is either 'I' or 'D'.

fn num_perms_di_sequence(s: String) -> i32 {
    let s: Vec<char> = s.chars().collect();
    let n = s.len() + 1;
    let mut memo = vec![vec![-1; n]; n];

    fn _num_perms_di_sequence(s: &[char], i: usize, j: usize, memo: &mut Vec<Vec<i32>>) -> i32 {
        if i == s.len() {
            return 1;
        }
        if memo[i][j] != -1 {
            return memo[i][j];
        }
        let initials = match s[i] {
            'D' => 0..j,
            'I' => j..s.len() - i, // j is excluded in the subproblem and all numbers that originally larger than j are decreased by 1.
            _ => unreachable!()
        };
        let result = initials.map(|x| _num_perms_di_sequence(s, i+1, x, memo))
            .reduce(|a, b| (a + b) % 1_000_000_007)
            .unwrap_or(0);
        memo[i][j] = result;
        result
    }

    (0..n).map(|x| _num_perms_di_sequence(&s, 0, x, &mut memo))
        .reduce(|a, b| (a + b) % 1_000_000_007)
        .unwrap_or(0)
}

fn main() {
    assert_eq!(num_perms_di_sequence("DID".to_string()), 5);
    assert_eq!(num_perms_di_sequence("D".to_string()), 1);
}

// faster than 100%, less memory than 100%.