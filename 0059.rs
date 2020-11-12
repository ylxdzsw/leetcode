// Given a positive integer n, generate a square matrix filled with elements from 1 to n2 in spiral order.

// Example:

// Input: 3
// Output:
// [
//  [ 1, 2, 3 ],
//  [ 8, 9, 4 ],
//  [ 7, 6, 5 ]
// ]

pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
    let mut result = vec![vec![0; n as _]; n as _];

    fn fill(n: i32, level: i32, mut count: i32, result: &mut [Vec<i32>]) {
        if count > (n * n) as _ {
            return
        }

        for i in level..=n-level-1 {
            result[level as usize][i as usize] = count;
            count += 1;
        }

        for i in level+1..=n-level-1 {
            result[i as usize][(n-level-1) as usize] = count;
            count += 1;
        }

        for i in (level..=n-level-2).rev() {
            result[(n-level-1) as usize][i as usize] = count;
            count += 1;
        }

        for i in (level+1..=n-level-2).rev() {
            result[i as usize][level as usize] = count;
            count += 1;
        }

        fill(n, level+1, count, result)
    };

    fill(n, 0, 1, &mut result);

    return result
}

fn main() {
    assert_eq!(generate_matrix(1), &[&[1]]);
    assert_eq!(generate_matrix(3), &[&[1,2,3],&[8,9,4],&[7,6,5]]);
}

// faster than 100.00%, less memory than 100.00%.