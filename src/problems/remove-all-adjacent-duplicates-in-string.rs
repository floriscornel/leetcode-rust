// @leetup=custom
// @leetup=info id=1047 lang=rust slug=remove-all-adjacent-duplicates-in-string

/*
* You are given a string `s` consisting of lowercase English letters. A
* duplicate removal consists of choosing two adjacent and equal
* letters and removing them.
*
* We repeatedly make duplicate removals on `s` until we no longer can.
*
* Return *the final string after all such duplicate removals have been made*. It
* can be proven that the answer is unique.
*
*
* Example 1:
*
* Input: s = "abbaca"
* Output: "ca"
* Explanation:
* For example, in "abbaca" we could remove "bb" since the letters are adjacent and
*  equal, and this is the only possible move.  The result of this move is that the
*  string is "aaca", of which only "aa" is possible, so the final string is "ca".
*
* Example 2:
*
* Input: s = "azxxzy"
* Output: "ay"
*
*
* Constraints:
*
* * `1 <= s.length <= 105`
* * `s` consists of lowercase English letters.
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
            if chars[i] == chars[i + 1] {
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

    pub fn remove_duplicates(s: String) -> String {
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
            Solution::remove_duplicates("abbaca".to_owned()),
            "ca".to_owned()
        )
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::remove_duplicates("azxxzy".to_owned()),
            "ay".to_owned()
        )
    }
}
// @leetup=inject:after_code
