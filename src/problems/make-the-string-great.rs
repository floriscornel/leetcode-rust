// @leetup=custom
// @leetup=info id=1544 lang=rust slug=make-the-string-great

/*
* Given a string `s` of lower and upper case English letters.
*
* A good string is a string which doesn't have two adjacent characters `s[i]`
* and `s[i + 1]` where:
*
* * `0 <= i <= s.length - 2`
* * `s[i]` is a lower-case letter and `s[i + 1]` is the same letter but in
*   upper-case or vice-versa.
*
* To make the string good, you can choose two adjacent characters that make
* the string bad and remove them. You can keep doing this until the string becomes
* good.
*
* Return *the string* after making it good. The answer is guaranteed to be unique
* under the given constraints.
*
* Notice that an empty string is also good.
*
*
* Example 1:
*
* Input: s = "leEeetcode"
* Output: "leetcode"
* Explanation: In the first step, either you choose i = 1 or i = 2, both will
* result "leEeetcode" to be reduced to "leetcode".
*
* Example 2:
*
* Input: s = "abBAcC"
* Output: ""
* Explanation: We have many possible scenarios, and all lead to the same answe
* r. For example:
* "abBAcC" --> "aAcC" --> "cC" --> ""
* "abBAcC" --> "abBA" --> "aA" --> ""
*
* Example 3:
*
* Input: s = "s"
* Output: "s"
*
*
* Constraints:
*
* * `1 <= s.length <= 100`
* * `s` contains only lower and upper case English letters.
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
    pub fn remove_chars(chars: &mut Vec<char>, from_idx: usize) -> usize {
        for i in from_idx..(chars.len() - 1) {
            let left = chars[i];
            let right = chars[i + 1];
            if (left.is_lowercase() && right.is_uppercase() && left.to_ascii_uppercase() == right)
                || (right.is_lowercase()
                    && left.is_uppercase()
                    && right.to_ascii_uppercase() == left)
            {
                chars.remove(i);
                chars.remove(i);
                if i == 0 {
                    return 0;
                } else {
                    return i - 1;
                }
            }
        }
        chars.len()
    }

    pub fn make_good(s: String) -> String {
        let mut chars = s.chars().collect::<Vec<char>>();
        let mut from_idx: usize = 0;
        while from_idx < chars.len() {
            from_idx = Solution::remove_chars(&mut chars, from_idx);
        }
        chars.iter().collect::<String>()
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
            Solution::make_good("leEeetcode".to_owned()),
            "leetcode".to_owned()
        )
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::make_good("abBAcC".to_owned()), "".to_owned())
    }

    #[test]
    fn example_3() {
        assert_eq!(Solution::make_good("Pp".to_owned()), "".to_owned())
    }
}
// @leetup=inject:after_code
