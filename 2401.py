from typing import List

def longestNiceSubarray(nums: List[int]) -> int:
    left, right, result = 0, 0, 1

    while True:
        right += 1

        if right == len(nums):
            return result

        x = right - 1
        while x >= left:
            if (nums[right] & nums[x]) != 0:
                left = x + 1
                break
            x -= 1
        else:
            result = max(result, right - left + 1)

assert longestNiceSubarray([1,3,8,48,10]) == 3
assert longestNiceSubarray([3,1,5,11,13]) == 1
