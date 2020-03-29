// Given a collection of candidate numbers (candidates) and a target number (target), find all unique combinations in candidates where the candidate numbers sums to target.

// Each number in candidates may only be used once in the combination.

// Note:

// All numbers (including target) will be positive integers.
// The solution set must not contain duplicate combinations.
// Example 1:

// Input: candidates = [10,1,2,7,6,1,5], target = 8,
// A solution set is:
// [
//   [1, 7],
//   [1, 2, 5],
//   [2, 6],
//   [1, 1, 6]
// ]
// Example 2:

// Input: candidates = [2,5,2,1,2], target = 5,
// A solution set is:
// [
//   [1,2,2],
//   [5]
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
            result.extend(_combination_sum(&candidates[1..], target - c).into_iter().map(|mut x| { x.push(c); x }))
        }

        let i = candidates.iter().rposition(|&x| x == c).unwrap(); // if we skip c, we must also skip other candidates that equals to c to prevent duplicate answer
        result.extend(_combination_sum(&candidates[i+1..], target));
        result
    }

    let mut candidates = candidates;
    candidates.sort_unstable_by_key(|x| -x); // we must sort because of we use it to skip same candidates
    _combination_sum(&candidates, target)
}

fn main() {
    let mut x = combination_sum(vec![10,1,2,7,6,1,5], 8);
    x.iter_mut().for_each(|x| x.sort_unstable());
    x.sort_unstable();
    let mut y = vec![&[1,7][..], &[1,2,5], &[2,6], &[1,1,6]];
    y.sort_unstable();
    assert_eq!(x, y);

    let mut x = combination_sum(vec![2,5,2,1,2], 5);
    x.iter_mut().for_each(|x| x.sort_unstable());
    x.sort_unstable();
    let mut y = vec![&[1,2,2][..], &[5]];
    y.sort_unstable();
    assert_eq!(x, y);
}

// faster than 100.00%, less memory than 100.00%.