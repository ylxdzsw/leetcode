def sumGame(num: str) -> bool:
    left = sum(int(x) if x != "?" else 4.5 for x in num[:len(num)//2])
    right = sum(int(x) if x != "?" else 4.5 for x in num[len(num)//2:])
    return left != right

assert sumGame("5023") == False
assert sumGame("25??") == True
assert sumGame("?3295???") == False
