// @leetup=custom
// @leetup=info id=76 lang=rust slug=minimum-window-substring

/*
* Given two strings `s` and `t` of lengths `m` and `n` respectively, return *the
* minimum window substring of *`s`* such that every character in *`t`*
* (including duplicates) is included in the window. If there is no such
* substring, return the empty string *`""`*.*
*
* The testcases will be generated such that the answer is unique.
*
* A substring is a contiguous sequence of characters within the string.
*
*
* Example 1:
*
* Input: s = "ADOBECODEBANC", t = "ABC"
* Output: "BANC"
* Explanation: The minimum window substring "BANC" includes 'A', 'B', and 'C'
* from string t.
*
* Example 2:
*
* Input: s = "a", t = "a"
* Output: "a"
* Explanation: The entire string s is the minimum window.
*
* Example 3:
*
* Input: s = "a", t = "aa"
* Output: ""
* Explanation: Both 'a's from t must be included in the window.
* Since the largest window of s only has one 'a', return empty string.
*
*
* Constraints:
*
* * `m == s.length`
* * `n == t.length`
* * `1 <= m, n <= 105`
* * `s` and `t` consist of uppercase and lowercase English letters.
*
*
* Follow up: Could you find an algorithm that runs in `O(m + n)` time?
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
    pub fn match_histogram(needle: &[i32; 52], haystack: &[i32; 52]) -> bool {
        for i in 0..52 {
            if needle[i] < haystack[i] {
                return false;
            }
        }
        true
    }

    pub fn find_window(
        s: &str,
        s_bytes: &[u8],
        haystack: &[i32; 52],
        size: usize,
    ) -> Option<(usize, usize)> {
        let idx = |c: u8| if c >= b'a' { c - b'a' + 26 } else { c - b'A' } as usize;

        let mut sliding_hist = [0; 52];
        let (mut i, mut j) = (0, 0);
        while j < size {
            sliding_hist[idx(s_bytes[j])] += 1;
            j += 1;
        }
        while j < s.len() {
            if Solution::match_histogram(&sliding_hist, haystack) {
                return Some((i, j));
            }
            sliding_hist[idx(s_bytes[i])] -= 1;
            sliding_hist[idx(s_bytes[j])] += 1;
            i += 1;
            j += 1;
        }

        if Solution::match_histogram(&sliding_hist, haystack) {
            Some((i, j))
        } else {
            None
        }
    }

    pub fn min_window(s: String, t: String) -> String {
        let idx = |c: u8| if c >= b'a' { c - b'a' + 26 } else { c - b'A' } as usize;
        // Build haystack hist
        let mut haystack = [0; 52];
        for c in t.bytes() {
            haystack[idx(c)] += 1;
        }
        // Check for each window size
        for window_size in t.len()..=s.len() {
            if let Some((i, j)) = Solution::find_window(&s, s.as_bytes(), &haystack, window_size) {
                return s[i..j].to_owned();
            }
        }
        "".to_owned()
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
            Solution::min_window("ADOBECODEBANC".to_owned(), "ABC".to_owned()),
            "BANC".to_owned()
        )
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::min_window("AAABBBCCCCZZZZ".to_owned(), "ABCZ".to_owned()),
            "ABBBCCCCZ".to_owned()
        )
    }

    #[test]
    fn example_3() {
        assert_eq!(
            Solution::min_window("ab".to_owned(), "A".to_owned()),
            "".to_owned()
        )
    }
}
// @leetup=inject:after_code
