from typing import List

def minCost(maxTime: int, edges: List[List[int]], passingFees: List[int]) -> int:
    adj_list = [ [] for _ in range(1000) ]
    for x, y, time in edges:
        adj_list[x].append((y, time))
        adj_list[y].append((x, time))

    cache = {} # (position, time) -> min cost

    def _minCost(current_position, remaining_time):
        if (current_position, remaining_time) in cache:
            return cache[(current_position, remaining_time)]

        best = 1000001

        for neighbour, time in adj_list[current_position]:
            if remaining_time >= time:
                if neighbour == len(passingFees) - 1:
                    fee = passingFees[neighbour]
                else:
                    fee = passingFees[neighbour] + _minCost(neighbour, remaining_time - time)
                if fee < best:
                    best = fee

        cache[(current_position, remaining_time)] = best

        return best

    fee = _minCost(0, maxTime)
    if fee > 1000000:
        return -1
    return passingFees[0] + fee

assert minCost(30, [[0,1,10],[1,2,10],[2,5,10],[0,3,1],[3,4,10],[4,5,15]], [5,1,2,20,20,3]) == 11
assert minCost(29, [[0,1,10],[1,2,10],[2,5,10],[0,3,1],[3,4,10],[4,5,15]], [5,1,2,20,20,3]) == 48
assert minCost(25, [[0,1,10],[1,2,10],[2,5,10],[0,3,1],[3,4,10],[4,5,15]], [5,1,2,20,20,3]) == -1
