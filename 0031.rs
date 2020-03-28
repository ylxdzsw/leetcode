// Implement next permutation, which rearranges numbers into the lexicographically next greater permutation of numbers.

// If such arrangement is not possible, it must rearrange it as the lowest possible order (ie, sorted in ascending order).

// The replacement must be in-place and use only constant extra memory.

// Here are some examples. Inputs are in the left-hand column and its corresponding outputs are in the right-hand column.

// 1,2,3 -> 1,3,2
// 3,2,1 -> 1,2,3
// 1,1,5 -> 1,5,1

fn next_permutation(nums: &mut Vec<i32>) {
    if let Some((i, _)) = nums.windows(2).enumerate().rev().find(|(_, w)| w[0] < w[1]) {
        // change i-th elment with the smallest one that is bigger
        let v = nums[i];
        let j = nums[i..].iter().enumerate().rev().find(|(_, &x)| x > v).unwrap().0 + i;
        nums[i] = nums[j];
        nums[j] = v;

        // then sort the rest
        nums[i+1..].sort_unstable()
    } else { // as requested
        nums.sort_unstable()
    }
}

fn main() {
    let mut a = vec![1, 2, 3];
    next_permutation(&mut a);
    assert_eq!(a, &[1, 3, 2]);

    let mut a = vec![3, 2, 1];
    next_permutation(&mut a);
    assert_eq!(a, &[1, 2, 3]);

    let mut a = vec![1, 1, 5];
    next_permutation(&mut a);
    assert_eq!(a, &[1, 5, 1]);
}

// faster than 100.00%, less memory than 100.00%.