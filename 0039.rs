// Given a set of candidate numbers (candidates) (without duplicates) and a target number (target), find all unique combinations in candidates where the candidate numbers sums to target.

// The same repeated number may be chosen from candidates unlimited number of times.

// Note:

// All numbers (including target) will be positive integers.
// The solution set must not contain duplicate combinations.
// Example 1:

// Input: candidates = [2,3,6,7], target = 7,
// A solution set is:
// [
//   [7],
//   [2,2,3]
// ]
// Example 2:

// Input: candidates = [2,3,5], target = 8,
// A solution set is:
// [
//   [2,2,2,2],
//   [2,3,3],
//   [3,5]
// ]

fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    #[allow(clippy::comparison_chain)]
    fn _combination_sum(candidates: &[i32], target: i32) -> Vec<Vec<i32>> {
        if target == 0 {
            return vec![vec![]]
        }

        let mut result = vec![];
        if candidates.is_empty() {
            return result
        }

        let c = candidates[0];
        if c <= target {
            result.extend(_combination_sum(candidates, target - c).into_iter().map(|mut x| { x.push(c); x }))
        }

        result.extend(_combination_sum(&candidates[1..], target));
        result
    }

    let mut candidates = candidates;
    candidates.sort_unstable_by_key(|x| -x); // try big numbers first to reduce branches
    _combination_sum(&candidates, target)
}

fn main() {
    let mut x = combination_sum(vec![2,3,6,7], 7);
    x.iter_mut().for_each(|x| x.sort_unstable());
    x.sort_unstable();
    let mut y = vec![&[7][..], &[2,2,3]];
    y.sort_unstable();
    assert_eq!(x, y);

    let mut x = combination_sum(vec![2,3,5], 8);
    x.iter_mut().for_each(|x| x.sort_unstable());
    x.sort_unstable();
    let mut y = vec![&[2,2,2,2][..], &[2,3,3], &[3,5]];
    y.sort_unstable();
    assert_eq!(x, y);
}

// faster than 100.00%, less memory than 100.00%.