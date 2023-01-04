// @leetup=custom
// @leetup=info id=290 lang=rust slug=word-pattern

/*
* Given a `pattern` and a string `s`, find if `s` follows the same pattern.
*
* Here follow means a full match, such that there is a bijection between a letter
* in `pattern` and a non-empty word in `s`.
*
*
* Example 1:
*
* Input: pattern = "abba", s = "dog cat cat dog"
* Output: true
*
* Example 2:
*
* Input: pattern = "abba", s = "dog cat cat fish"
* Output: false
*
* Example 3:
*
* Input: pattern = "aaaa", s = "dog cat cat dog"
* Output: false
*
*
* Constraints:
*
* * `1 <= pattern.length <= 300`
* * `pattern` contains only lower-case English letters.
* * `1 <= s.length <= 3000`
* * `s` contains only lowercase English letters and spaces `' '`.
* * `s` does not contain any leading or trailing spaces.
* * All the words in `s` are separated by a single space.
*
*/
// @leetup=custom
// @leetup=inject:before_code_ex
// before_code_exclude
#![allow(dead_code, unused_variables)]
struct Solution {}
// @leetup=inject:before_code_ex

// @leetup=code
use std::collections::HashMap;

impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let mut pattern_to_string = HashMap::<&u8, &str>::new();
        let mut string_to_pattern = HashMap::<&str, &u8>::new();

        if pattern.len() != s.split_whitespace().count() {
            return false;
        }

        pattern
            .as_bytes()
            .iter()
            .zip(s.split_whitespace())
            .map(|(c, s)| {
                let found_string = pattern_to_string.entry(c).or_insert(s);
                let found_pattern = string_to_pattern.entry(s).or_insert(c);
                *found_string == s && *found_pattern == c
            })
            .all(|x| x)
    }
}
// @leetup=code

// @leetup=inject:after_code
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let pattern = "abba".to_string();
        let s = "dog cat cat dog".to_string();
        let excpected = true;
        assert_eq!(Solution::word_pattern(pattern, s), excpected);
    }

    #[test]
    fn example_2() {
        let pattern = "abba".to_string();
        let s = "dog cat cat fish".to_string();
        let excpected = false;
        assert_eq!(Solution::word_pattern(pattern, s), excpected);
    }

    #[test]
    fn example_3() {
        let pattern = "aaaa".to_string();
        let s = "dog cat cat dog".to_string();
        let excpected = false;
        assert_eq!(Solution::word_pattern(pattern, s), excpected);
    }

    #[test]
    fn example_4() {
        let pattern = "abba".to_string();
        let s = "dog dog dog dog".to_string();
        let excpected = false;
        assert_eq!(Solution::word_pattern(pattern, s), excpected);
    }

    #[test]
    fn example_5() {
        let pattern = "aaa".to_string();
        let s = "aa aa aa aa".to_string();
        let excpected = false;
        assert_eq!(Solution::word_pattern(pattern, s), excpected);
    }
}
// @leetup=inject:after_code
