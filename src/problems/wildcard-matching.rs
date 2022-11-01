// @leetup=custom
// @leetup=info id=44 lang=rust slug=wildcard-matching

/*
* Given an input string (`s`) and a pattern (`p`), implement wildcard pattern
* matching with support for `'?'` and `'*'` where:
*
* * `'?'` Matches any single character.
* * `'*'` Matches any sequence of characters (including the empty sequence).
*
* The matching should cover the entire input string (not partial).
*
*
* Example 1:
*
* Input: s = "aa", p = "a"
* Output: false
* Explanation: "a" does not match the entire string "aa".
*
* Example 2:
*
* Input: s = "aa", p = "*"
* Output: true
* Explanation: '*' matches any sequence.
*
* Example 3:
*
* Input: s = "cb", p = "?a"
* Output: false
* Explanation: '?' matches 'c', but the second letter is 'a', which does not m
* atch 'b'.
*
*
* Constraints:
*
* * `0 <= s.length, p.length <= 2000`
* * `s` contains only lowercase English letters.
* * `p` contains only lowercase English letters, `'?'` or `'*'`.
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
    pub fn check(
        s: &[u8],
        p: &[u8],
        i: usize,
        j: usize,
        set: &mut HashSet<(usize, usize)>,
    ) -> bool {
        match set.insert((i, j)) {
            false => false,
            true => match (s.get(i), p.get(j)) {
                (None, None) => true,
                (Some(_), None) => false,
                (None, Some(x)) => *x == b'*' && Self::check(s, p, i, j + 1, set),
                (Some(a), Some(b)) => match *b {
                    b'?' => Self::check(s, p, i + 1, j + 1, set),
                    b'a'..=b'z' => a == b && Self::check(s, p, i + 1, j + 1, set),
                    b'*' => Self::check(s, p, i, j + 1, set) || Self::check(s, p, i + 1, j, set),
                    unknown => panic!("Unknown character '{unknown}' found in pattern"),
                },
            },
        }
    }

    pub fn is_match(s: String, p: String) -> bool {
        Self::check(s.as_bytes(), p.as_bytes(), 0, 0, &mut HashSet::new())
    }
}
// @leetup=code

// @leetup=inject:after_code
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        assert!(!Solution::is_match("aa".to_owned(), "a".to_owned()))
    }

    #[test]
    fn example_2() {
        assert!(Solution::is_match("aa".to_owned(), "*".to_owned()))
    }

    #[test]
    fn example_3() {
        assert!(!Solution::is_match("cb".to_owned(), "?a".to_owned()))
    }

    #[test]
    fn example_4() {
        assert!(!Solution::is_match("acdcb".to_owned(), "a*c?b".to_owned()))
    }

    #[test]
    fn example_5() {
        assert!(!Solution::is_match(
            "bbbbbbbabbaabbabbbbaaabbabbabaaabbababbbabbbabaaabaab".to_owned(),
            "b*b*ab**ba*b**b***bba".to_owned()
        ))
    }
}
// @leetup=inject:after_code
