// @leetup=custom
// @leetup=info id=17 lang=rust slug=letter-combinations-of-a-phone-number

/*
* Given a string containing digits from `2-9` inclusive, return all possible
* letter combinations that the number could represent. Return the answer in any
* order.
*
* A mapping of digit to letters (just like on the telephone buttons) is given
* below. Note that 1 does not map to any letters.
*
*
*
* Example 1:
*
* Input: digits = "23"
* Output: ["ad","ae","af","bd","be","bf","cd","ce","cf"]
*
* Example 2:
*
* Input: digits = ""
* Output: []
*
* Example 3:
*
* Input: digits = "2"
* Output: ["a","b","c"]
*
*
* Constraints:
*
* * `0 <= digits.length <= 4`
* * `digits[i]` is a digit in the range `['2', '9']`.
*
*/
// @leetup=custom
// @leetup=inject:before_code_ex
// before_code_exclude
#![allow(dead_code, unused_variables)]
struct Solution {}
// @leetup=inject:before_code_ex

// @leetup=code

impl Solution {
    fn permute(options: &Vec<String>, chars: Vec<char>) -> Vec<String> {
        let mut output = Vec::new();
        for c in chars {
            for o in options {
                let mut tmp = String::from(o);
                tmp.push(c);
                output.push(tmp);
            }
        }
        output
    }

    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return Vec::new();
        }

        let phone_keyboard_mapping = |x: char| match x {
            '2' => vec!['a', 'b', 'c'],
            '3' => vec!['d', 'e', 'f'],
            '4' => vec!['g', 'h', 'i'],
            '5' => vec!['j', 'k', 'l'],
            '6' => vec!['m', 'n', 'o'],
            '7' => vec!['p', 'q', 'r', 's'],
            '8' => vec!['t', 'u', 'v'],
            '9' => vec!['w', 'x', 'y', 'z'],
            _ => vec!['_'],
        };

        let mut output: Vec<String> = vec!["".to_owned()];
        for input_char in digits.chars() {
            output = Solution::permute(&output, phone_keyboard_mapping(input_char));
        }
        output
    }
}
// @leetup=code

// @leetup=inject:after_code
#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::Solution;

    #[test]
    fn example_1() {
        let digits = "23".to_owned();
        let output: Vec<String> = vec![
            "ad".to_owned(),
            "ae".to_owned(),
            "af".to_owned(),
            "bd".to_owned(),
            "be".to_owned(),
            "bf".to_owned(),
            "cd".to_owned(),
            "ce".to_owned(),
            "cf".to_owned(),
        ];
        assert_eq!(
            HashSet::<String>::from_iter(Solution::letter_combinations(digits)),
            HashSet::<String>::from_iter(output)
        );
    }

    #[test]
    fn example_2() {
        let digits = "".to_owned();
        let output = Vec::<&str>::new();
        assert_eq!(Solution::letter_combinations(digits), output);
    }

    #[test]
    fn example_3() {
        let digits = "2".to_owned();
        let output: Vec<String> = vec!["a".to_owned(), "b".to_owned(), "c".to_owned()];
        assert_eq!(
            HashSet::<String>::from_iter(Solution::letter_combinations(digits)),
            HashSet::<String>::from_iter(output)
        );
    }
}
// @leetup=inject:after_code
