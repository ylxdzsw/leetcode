// Given an integer n, count the total number of digit 1 appearing in all non-negative integers less than or equal to n.

// Example:

// Input: 13
// Output: 6 
// Explanation: Digit 1 occurred in the following numbers: 1, 10, 11, 12, 13.

fn count_digit_one(n: i32) -> i32 {
    fn _count_digit_one(n: usize) -> usize {
        let mut maxlevel = 0;
        while n / 10usize.pow(maxlevel) > 0 {
            maxlevel += 1;
        }
        match maxlevel {
            0 => return 0,
            1 => return 1,
            _ => {}
        }
    
        let most_sigificant_digit = n / 10usize.pow(maxlevel - 1);
        let mut count = 0;

        // the ones that comes from the most significant digit
        count += if most_sigificant_digit == 1 {
            n % 10usize.pow(maxlevel - 1) + 1
        } else {
            10usize.pow(maxlevel - 1)
        };

        // the ones that comes from less significant digits of `maxlevel` digits numbers
        count += most_sigificant_digit * (maxlevel - 1) as usize * 10usize.pow(maxlevel - 2);
    
        // the ones that comes from the numbers that has less digits
        count += _count_digit_one(n % 10usize.pow(maxlevel - 1));

        count
    }

    if n <= 0 {
        return 0
    }

    _count_digit_one(n as _) as _
}

fn main() {
    assert_eq!(count_digit_one(13), 6)
}

// faster than 100.00%, less memory than 100.00%.