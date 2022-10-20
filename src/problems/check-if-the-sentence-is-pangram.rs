// @leetup=custom
// @leetup=info id=1832 lang=rust slug=check-if-the-sentence-is-pangram

/*
* A pangram is a sentence where every letter of the English alphabet appears
* at least once.
*
* Given a string `sentence` containing only lowercase English letters, return*
* *`true`* if *`sentence`* is a pangram, or *`false`* otherwise.*
*
*
* Example 1:
*
* Input: sentence = "thequickbrownfoxjumpsoverthelazydog"
* Output: true
* Explanation: sentence contains at least one of every letter of the English a
* lphabet.
*
* Example 2:
*
* Input: sentence = "leetcode"
* Output: false
*
*
* Constraints:
*
* * `1 <= sentence.length <= 1000`
* * `sentence` consists of lowercase English letters.
*
*/
// @leetup=custom
// @leetup=inject:before_code_ex
// before_code_exclude
#![allow(dead_code, unused_variables)]

struct Solution {}
// @leetup=inject:before_code_ex

// @leetup=code
use std::collections::HashSet;

impl Solution {
    pub fn check_if_pangram(sentence: String) -> bool {
        let mut set = HashSet::<char>::new();
        for next_character in sentence.chars() {
            if !set.contains(&next_character) {
                set.insert(next_character);
                if set.len() == 26 {
                    return true;
                }
            }
        }
        false
    }
}
// @leetup=code

// @leetup=inject:after_code
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        assert!(Solution::check_if_pangram(
            "thequickbrownfoxjumpsoverthelazydog".to_owned()
        ))
    }

    #[test]
    fn example_2() {
        assert!(!Solution::check_if_pangram("leetcode".to_owned()))
    }
}
// @leetup=inject:after_code
