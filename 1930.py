def countPalindromicSubsequence(s: str) -> int:
    counts = {} # end letter => number of palindroms
    for left in range(len(s) - 2):
        if s[left] in counts:
            continue
        right = len(s) - 1
        while right > left + 1:
            if s[right] == s[left]:
                break
            right -= 1
        else:
            continue
        unique = set()
        for mid in range(left+1, right):
            unique.add(s[mid])
        counts[s[left]] = len(unique)
    return sum(v for v in counts.values())

assert countPalindromicSubsequence("aabca") == 3
assert countPalindromicSubsequence("adc") == 0
assert countPalindromicSubsequence("bbcbaba") == 4
