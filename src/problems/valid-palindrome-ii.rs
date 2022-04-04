// @leetup=custom
// @leetup=info id=680 lang=rust slug=valid-palindrome-ii

/*
* Given a string `s`, return `true` *if the *`s`* can be palindrome after deleting
* at most one character from it*.
*
*
* Example 1:
*
* Input: s = "aba"
* Output: true
*
* Example 2:
*
* Input: s = "abca"
* Output: true
* Explanation: You could delete the character 'c'.
*
* Example 3:
*
* Input: s = "abc"
* Output: false
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
    fn helper(arr: &[char], skip: usize) -> bool {
        let (mut i, mut j) = (0, arr.len() - 1);
        while i < j {
            if i == skip {
                i += 1;
            }
            if j == skip {
                j -= 1;
            }
            if arr[i] != arr[j] {
                return false;
            }
            i += 1;
            j -= 1;
        }
        true
    }

    pub fn valid_palindrome(s: String) -> bool {
        let arr: Vec<char> = s.chars().collect();
        let (mut i, mut j) = (0, arr.len() - 1);
        while i < j {
            if arr[i] != arr[j] {
                return Solution::helper(&arr, i) || Solution::helper(&arr, j);
            } else {
                i += 1;
                j -= 1;
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
        assert!(Solution::valid_palindrome("aba".to_owned()));
    }

    #[test]
    fn example_2() {
        assert!(Solution::valid_palindrome("abca".to_owned()));
    }

    #[test]
    fn example_3() {
        assert!(!Solution::valid_palindrome("abc".to_owned()));
    }

    #[test]
    fn example_4() {
        assert!(Solution::valid_palindrome("a".to_owned()));
    }

    #[test]
    fn example_5() {
        assert!(Solution::valid_palindrome("ab".to_owned()));
    }

    #[test]
    fn example_6() {
        assert!(Solution::valid_palindrome("ba".to_owned()));
    }

    #[test]
    fn example_7() {
        assert!(Solution::valid_palindrome(
            "ebcbbececabbacecbbcbe".to_owned()
        ));
    }
}
// @leetup=inject:after_code
