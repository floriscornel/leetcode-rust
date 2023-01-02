// @leetup=custom
// @leetup=info id=520 lang=rust slug=detect-capital

/*
* We define the usage of capitals in a word to be right when one of the following
* cases holds:
*
* * All letters in this word are capitals, like `"USA"`.
* * All letters in this word are not capitals, like `"leetcode"`.
* * Only the first letter in this word is capital, like `"Google"`.
*
* Given a string `word`, return `true` if the usage of capitals in it is right.
*
*
* Example 1:
*
* Input: word = "USA"
* Output: true
*
* Example 2:
*
* Input: word = "FlaG"
* Output: false
*
*
* Constraints:
*
* * `1 <= word.length <= 100`
* * `word` consists of lowercase and uppercase English letters.
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
    pub fn detect_capital_use(word: String) -> bool {
        let mut all_upper = true;
        for (idx, letter) in word.chars().enumerate() {
            match (idx, letter.is_uppercase(), all_upper) {
                (1.., true, false) => return false,
                (2.., false, true) => return false,
                (_, false, true) => all_upper = false,
                _ => {}
            }
        }
        true
    }
}
// @leetup=code

// @leetup=inject:after_code
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let word = "USA".to_string();
        let expected = true;
        assert_eq!(Solution::detect_capital_use(word), expected);
    }

    #[test]
    fn example_2() {
        let expected = [
            ("flag", true),
            ("Flag", true),
            ("FLAG", true),
            ("FLag", false),
            ("FLAg", false),
            ("FLaG", false),
            ("FlaG", false),
            ("flaG", false),
            ("flAg", false),
            ("fLag", false),
            ("fLAG", false),
        ];

        for (word, expected) in expected.iter() {
            assert_eq!(
                Solution::detect_capital_use(word.to_string()),
                *expected,
                "word: {}",
                word
            );
        }
    }
}
// @leetup=inject:after_code
