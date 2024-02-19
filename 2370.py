def longestIdealString(s: str, k: int) -> int:
    best = [0] * 26
    for c in s:
        c = ord(c) - ord('a')
        best[c] = max(best[i] + 1 for i in range(max(0, c-k), min(26, c+k+1)))
    return max(best)

assert longestIdealString("acfgbd", 2) == 4
assert longestIdealString("abcd", 3) == 4
