// @leetup=custom
// @leetup=info id=242 lang=rust slug=valid-anagram

/*
* Given two strings `s` and `t`, return `true` *if* `t` *is an anagram of* `s`*,
* and* `false` *otherwise*.
*
* An Anagram is a word or phrase formed by rearranging the letters of a
* different word or phrase, typically using all the original letters exactly once.
*
*
* Example 1:
*
* Input: s = "anagram", t = "nagaram"
* Output: true
*
* Example 2:
*
* Input: s = "rat", t = "car"
* Output: false
*
*
* Constraints:
*
* * `1 <= s.length, t.length <= 5 * 104`
* * `s` and `t` consist of lowercase English letters.
*
*
* Follow up: What if the inputs contain Unicode characters? How would you
* adapt your solution to such a case?
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
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut histogram = HashMap::<char, i32>::with_capacity(26);
        for char in s.chars() {
            *histogram.entry(char).or_insert(0) += 1;
        }
        for char in t.chars() {
            let char_count = histogram.entry(char).or_insert(0);
            if *char_count == 0 {
                return false;
            }
            *char_count -= 1;
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
        assert!(Solution::is_anagram(
            "anagram".to_owned(),
            "nagaram".to_owned()
        ));
    }

    #[test]
    fn example_2() {
        assert!(!Solution::is_anagram("rat".to_owned(), "cat".to_owned()));
    }

    #[test]
    fn example_3() {
        assert!(!Solution::is_anagram("ab".to_owned(), "a".to_owned()));
    }
}
// @leetup=inject:after_code
