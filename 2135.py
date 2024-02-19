from typing import List

def wordCount(startWords: List[str], targetWords: List[str]) -> int:
    result = 0
    startWords = set(tuple(sorted(x)) for x in startWords)
    for target in targetWords:
        target = sorted(target)
        for i in range(len(target)):
            if tuple(target[:i] + target[i+1:]) in startWords:
                result += 1
                break
    return result

assert wordCount(["ant","act","tack"], ["tack","act","acti"]) == 2
assert wordCount(["ab","a"], ["abc","abcd"]) == 1
