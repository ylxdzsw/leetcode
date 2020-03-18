// Given a string containing digits from 2-9 inclusive, return all possible letter combinations that the number could represent.

// A mapping of digit to letters (just like on the telephone buttons) is given below. Note that 1 does not map to any letters.

// Example:

// Input: "23"
// Output: ["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"].
// Note:

// Although the above answer is in lexicographical order, your answer could be in any order you want.

fn letter_combinations(digits: String) -> Vec<String> {
    let letter_map = [
        &[][..], &[],
        &['a', 'b', 'c'],
        &['d', 'e', 'f'],
        &['g', 'h', 'i'],
        &['j', 'k', 'l'],
        &['m', 'n', 'o'],
        &['p', 'q', 'r', 's'],
        &['t', 'u', 'v'],
        &['w', 'x', 'y', 'z']
    ];

    fn _letter_combinations(letter_map: &[&[char]], digits: &[usize]) -> Vec<String> {
        match digits.len() {
            0 => vec![],
            1 => letter_map[digits[0]].iter().map(|c| c.to_string()).collect(),
            _ => {
                let list = letter_map[*digits.last().unwrap()];
                _letter_combinations(letter_map, &digits[..digits.len()-1]).into_iter().flat_map(|x| {
                    list.iter().map(move |c| format!("{}{}", x, c))
                }).collect()
            }
        }
    }

    _letter_combinations(&letter_map, &digits.chars().map(|x| x.to_digit(10).unwrap() as usize).collect::<Vec<_>>())
}

fn main() {
    assert_eq!(letter_combinations("23".to_string()), &["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]);
}

// faster than 100.00%, less memory than 100.00%.