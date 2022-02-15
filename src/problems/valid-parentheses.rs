// @leetup=custom
// @leetup=info id=20 lang=rust slug=valid-parentheses

/*
* Given a string `s` containing just the characters `'('`, `')'`, `'{'`, `'}'`,
* `'['` and `']'`, determine if the input string is valid.
*
* An input string is valid if:
*
* 1. Open brackets must be closed by the same type of brackets.
* 2. Open brackets must be closed in the correct order.
*
*
* Example 1:
*
* Input: s = "()"
* Output: true
*
* Example 2:
*
* Input: s = "()[]{}"
* Output: true
*
* Example 3:
*
* Input: s = "(]"
* Output: false
*
*
* Constraints:
*
* * `1 <= s.length <= 104`
* * `s` consists of parentheses only `'()[]{}'`.
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
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = Vec::new();
        for read in s.chars() {
            match read {
                '(' | '[' | '{' => stack.push(read),
                ')' | ']' | '}' => {
                    if let Some(popped) = stack.pop() {
                        if (popped == '(' && read != ')')
                            || (popped == '[' && read != ']')
                            || (popped == '{' && read != '}')
                        {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
                _ => return false,
            }
        }
        stack.is_empty()
    }
}
// @leetup=code

// @leetup=inject:after_code
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        assert!(Solution::is_valid("()".to_string()));
    }

    #[test]
    fn example_2() {
        assert!(Solution::is_valid("()[]{}".to_string()));
    }

    #[test]
    fn example_3() {
        assert!(!Solution::is_valid("(]".to_string()));
    }
}
// @leetup=inject:after_code
