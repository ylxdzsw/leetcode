from typing import List

def sellingWood(m: int, n: int, prices: List[List[int]]) -> int:
    price_dict = { (m, n): p for m, n, p in prices }
    cache = {}

    def f(m, n):
        if (m, n) in cache:
            return cache[(m, n)]

        best = price_dict.get((m, n), 0)

        for s in range(1, m//2+1):
            best = max(best, f(s, n) + f(m-s, n))

        for s in range(1, n//2+1):
            best = max(best, f(m, s) + f(m, n-s))

        cache[(m, n)] = best
        return best

    return f(m, n)

assert sellingWood(3, 5, [[1,4,2],[2,2,7],[2,1,3]]) == 19
assert sellingWood(4, 6, [[3,2,10],[1,4,2],[4,1,3]]) == 32
