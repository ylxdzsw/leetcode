def checkPowersOfThree(n: int) -> bool:
    def f(n, limit):
        for i in range(limit + 1):
            if pow(3, i) > n:
                if i - 1 == limit:
                    return False
                return f(n - pow(3, i - 1), i - 1)

            if pow(3, i) == n:
                if i == limit:
                    return False
                return True

        return False

    return f(n, n)

assert checkPowersOfThree(12)
assert checkPowersOfThree(91)
assert not checkPowersOfThree(21)
