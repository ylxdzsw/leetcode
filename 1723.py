from typing import List

def minimumTimeRequired(jobs: List[int], k: int) -> int:
    jobs = list(reversed(sorted(jobs)))
    best = sum(jobs)

    def f(assignment, working_times):
        nonlocal best

        if len(assignment) == len(jobs):
            best = min(max(working_times), best)
            return

        tried = set()
        for i in range(k):
            if working_times[i] in tried:
                continue
            else:
                tried.add(working_times[i])

            assignment.append(i)
            working_times[i] += jobs[len(assignment) - 1]
            if working_times[i] < best:
                f(assignment, working_times)
            assignment.pop()
            working_times[i] -= jobs[len(assignment)]

    f([], [0] * k)

    return best

assert minimumTimeRequired([3,2,3], 3) == 3
assert minimumTimeRequired([1,2,4,7,8], 2) == 11
