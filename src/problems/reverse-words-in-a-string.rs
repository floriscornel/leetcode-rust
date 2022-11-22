// @leetup=custom
// @leetup=info id=151 lang=rust slug=reverse-words-in-a-string

/*
* Given an input string `s`, reverse the order of the words.
*
* A word is defined as a sequence of non-space characters. The words in
* `s` will be separated by at least one space.
*
* Return *a string of the words in reverse order concatenated by a single space.*
*
* Note that `s` may contain leading or trailing spaces or multiple spaces between
* two words. The returned string should only have a single space separating the
* words. Do not include any extra spaces.
*
*
* Example 1:
*
* Input: s = "the sky is blue"
* Output: "blue is sky the"
*
* Example 2:
*
* Input: s = "  hello world  "
* Output: "world hello"
* Explanation: Your reversed string should not contain leading or trailing spa
* ces.
*
* Example 3:
*
* Input: s = "a good   example"
* Output: "example good a"
* Explanation: You need to reduce multiple spaces between two words to a singl
* e space in the reversed string.
*
*
* Constraints:
*
* * `1 <= s.length <= 104`
* * `s` contains English letters (upper-case and lower-case), digits, and spaces
*   `' '`.
* * There is at least one word in `s`.
*
*
* Follow-up: If the string data type is mutable in your language, can you solve it
* in-place with `O(1)` extra space?
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
    pub fn reverse_words(s: String) -> String {
        s.split(' ')
            .rev()
            .filter(|&x| !x.is_empty())
            .collect::<Vec<&str>>()
            .join(" ")
    }
}
// @leetup=code

// @leetup=inject:after_code
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::reverse_words("the sky is blue".to_owned()),
            "blue is sky the".to_owned()
        )
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::reverse_words("  hello world  ".to_owned()),
            "world hello".to_owned()
        )
    }

    #[test]
    fn example_3() {
        assert_eq!(
            Solution::reverse_words("a good   example".to_owned()),
            "example good a".to_owned()
        )
    }
}
// @leetup=inject:after_code
