// Your music player contains N different songs and she wants to listen to L (not necessarily different) songs during your trip.  You create a playlist so that:

// Every song is played at least once
// A song can only be played again only if K other songs have been played
// Return the number of possible playlists.  As the answer can be very large, return it modulo 10^9 + 7.

// Example 1:

// Input: N = 3, L = 3, K = 1
// Output: 6
// Explanation: There are 6 possible playlists. [1, 2, 3], [1, 3, 2], [2, 1, 3], [2, 3, 1], [3, 1, 2], [3, 2, 1].
// Example 2:

// Input: N = 2, L = 3, K = 0
// Output: 6
// Explanation: There are 6 possible playlists. [1, 1, 2], [1, 2, 1], [2, 1, 1], [2, 2, 1], [2, 1, 2], [1, 2, 2]
// Example 3:

// Input: N = 2, L = 3, K = 1
// Output: 2
// Explanation: There are 2 possible playlists. [1, 2, 1], [2, 1, 2]

// Note:

// 0 <= K < N <= L <= 100

// the dp solution in the official Leetcode solution. By trakcing `j` (the distinct songs played) we can easily deal with the requirement of everysong must be played at least once.
fn num_music_playlists(n: i32, l: i32, k: i32) -> i32 {
    let n = n as usize;
    let l = l as usize;
    let k = k as usize;

    let mut dp = vec![vec![0; n+1]; l+1];
    dp[0][0] = 1;

    for i in 1..=l {
        for j in 1..=n {
            dp[i][j] += dp[i-1][j-1] * (n-j+1); // the last song is not played before
            dp[i][j] += dp[i-1][j] * j.saturating_sub(k) ; // the last song was played
            dp[i][j] %= 10usize.pow(9) + 7
        }
    }

    dp[l][n] as _
}

fn main() {
    assert_eq!(num_music_playlists(3, 3, 1), 6);
    assert_eq!(num_music_playlists(2, 3, 0), 6);
    assert_eq!(num_music_playlists(2, 3, 1), 2);
}

// faster than 100.00%, less memory than 100.00%.