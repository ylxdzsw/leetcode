def numberOfWays(startPos: int, endPos: int, k: int) -> int:
    cache = {}
    mod = 10**9 + 7

    def _numberOfWays(startPos, k):
        if (startPos, k) in cache:
            return cache[(startPos, k)]

        dist = abs(startPos - endPos)
        if k < dist:
            result = 0

        elif k == dist:
            result = 1

        else:
            result = _numberOfWays(startPos + 1, k - 1) % mod + _numberOfWays(startPos - 1, k - 1) % mod

        cache[(startPos, k)] = result
        return result

    return _numberOfWays(startPos, k) % mod

assert numberOfWays(1, 2, 3) == 3
assert numberOfWays(2, 5, 10) == 0
