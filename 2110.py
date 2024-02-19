from typing import List

def getDescentPeriods(prices: List[int]) -> int:
    result = 0
    last_start = 0
    for i in range(len(prices)):
        diff = i - last_start
        if prices[i] == prices[last_start] - diff:
            result += diff
        else:
            last_start = i
        result += 1
    return result

assert getDescentPeriods([3,2,1,4]) == 7
assert getDescentPeriods([8,6,7,7]) == 4
assert getDescentPeriods([1]) == 1
