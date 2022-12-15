// @leetup=custom
// @leetup=info id=1143 lang=rust slug=longest-common-subsequence

/*
* Given two strings `text1` and `text2`, return *the length of their longest
* common subsequence. *If there is no common subsequence, return `0`.
*
* A subsequence of a string is a new string generated from the original string
* with some characters (can be none) deleted without changing the relative order
* of the remaining characters.
*
* * For example, `"ace"` is a subsequence of `"abcde"`.
*
* A common subsequence of two strings is a subsequence that is common to both
* strings.
*
*
* Example 1:
*
* Input: text1 = "abcde", text2 = "ace"
* Output: 3
* Explanation: The longest common subsequence is "ace" and its length is 3.
*
* Example 2:
*
* Input: text1 = "abc", text2 = "abc"
* Output: 3
* Explanation: The longest common subsequence is "abc" and its length is 3.
*
* Example 3:
*
* Input: text1 = "abc", text2 = "def"
* Output: 0
* Explanation: There is no such common subsequence, so the result is 0.
*
*
* Constraints:
*
* * `1 <= text1.length, text2.length <= 1000`
* * `text1` and `text2` consist of only lowercase English characters.
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
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let mut dp: Vec<Vec<usize>> = vec![vec![0; text2.len() + 1]; text1.len() + 1];
        for (i, &a) in text1.as_bytes().iter().enumerate() {
            for (j, &b) in text2.as_bytes().iter().enumerate() {
                let (top_left, top, left) = (dp[i][j], dp[i][j + 1], dp[i + 1][j]);
                dp[i + 1][j + 1] = if a == b { top_left + 1 } else { top.max(left) }
            }
        }
        dp[text1.len()][text2.len()] as i32
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
            Solution::longest_common_subsequence("abcde".to_string(), "ace".to_string()),
            3
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::longest_common_subsequence("abc".to_string(), "abc".to_string()),
            3
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            Solution::longest_common_subsequence("abc".to_string(), "def".to_string()),
            0
        );
    }
}
// @leetup=inject:after_code
