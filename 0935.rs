// A chess knight can move as indicated in the chess diagram below:

// This time, we place our chess knight on any numbered key of a phone pad (indicated above), and the knight makes N-1 hops.  Each hop must be from one key to another numbered key.

// Each time it lands on a key (including the initial placement of the knight), it presses the number of that key, pressing N digits total.

// How many distinct numbers can you dial in this manner?

// Since the answer may be large, output the answer modulo 10^9 + 7.

// Example 1:

// Input: 1
// Output: 10
// Example 2:

// Input: 2
// Output: 20
// Example 3:

// Input: 3
// Output: 46

// Note:

// 1 <= N <= 5000

fn knight_dialer(n: i32) -> i32 {
    const modulo: usize = 1000000007;
    fn _knight_dialer(n: usize) -> [usize; 10] {
        if n == 1 {
            return [1; 10]
        }

        let x = _knight_dialer(n - 1);
        let mut res = [
            x[4] + x[6],
            x[6] + x[8],
            x[7] + x[9],
            x[4] + x[8],
            x[3] + x[9] + x[0],
            0,
            x[1] + x[7] + x[0],
            x[2] + x[6],
            x[1] + x[3],
            x[2] + x[4]
        ];

        for r in res.iter_mut() {
            *r = *r % modulo
        }

        res
    }

    (_knight_dialer(n as _).iter().sum::<usize>() % modulo) as _
}

fn main() {
    assert_eq!(knight_dialer(1), 10);
    assert_eq!(knight_dialer(2), 20);
    assert_eq!(knight_dialer(3), 46);
}

// faster than 100.00%, less memory than 100.00%.