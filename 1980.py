from typing import List

def findDifferentBinaryString(nums: List[str]) -> str:
    output = ''
    for i in range(len(nums)):
        if nums[i][i] == '1':
            output += '0'
        else:
            output += '1'
    return output

assert findDifferentBinaryString(["01", "10"]) not in ["01", "10"]
assert findDifferentBinaryString(["00", "01"]) not in ["00", "01"]
assert findDifferentBinaryString(["111", "011", "001"]) not in ["111", "011", "001"]
