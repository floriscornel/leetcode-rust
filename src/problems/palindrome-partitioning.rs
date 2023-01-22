// @leetup=custom
// @leetup=info id=131 lang=rust slug=palindrome-partitioning

/*
* Given a string `s`, partition `s` such that every substring of the partition is
* a palindrome. Return *all possible palindrome partitioning of *`s`.
*
*
* Example 1:
*
* Input: s = "aab"
* Output: [["a","a","b"],["aa","b"]]
*
* Example 2:
*
* Input: s = "a"
* Output: [["a"]]
*
*
* Constraints:
*
* * `1 <= s.length <= 16`
* * `s` contains only lowercase English letters.
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
    fn is_palindrome(s: &str) -> bool {
        let mut chars = s.chars();
        while let (Some(c1), Some(c2)) = (chars.next(), chars.next_back()) {
            if c1 != c2 {
                return false;
            }
        }
        true
    }

    fn valid_partition(s: &str, mask: u16) -> Option<Vec<String>> {
        let mut result = vec!["".to_string()];
        for (idx, char) in s.chars().enumerate() {
            // iterate all chars
            let last_substring = result.last_mut().unwrap();
            last_substring.push(char); // Add char to last string
            if mask & (1 << idx) != 0 {
                if last_substring.len() > 1 && !Self::is_palindrome(last_substring) {
                    // if the previous substring is not palindrome, return None
                    return None;
                }
                result.push("".to_string()); // Add a new empty string
            }
        }
        let last_substring = result.last().unwrap();
        if last_substring.len() > 1 && !Self::is_palindrome(last_substring) {
            // if the last substring is not palindrome, return None
            None
        } else {
            Some(result)
        }
    }

    pub fn partition(s: String) -> Vec<Vec<String>> {
        let max = 1 << (s.len() - 1); // For a string of length n, there are 2^(n-1) possible partitions
        let mut result = vec![];
        for mask in 0..max {
            if let Some(answer) = Self::valid_partition(&s, mask) {
                result.push(answer);
            }
        }
        result
    }
}
// @leetup=code

// @leetup=inject:after_code
#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::Solution;

    #[test]
    fn example_1() {
        let s = "aab".to_string();
        let result = vec![vec!["a", "a", "b"], vec!["aa", "b"]];

        let a = Solution::partition(s);
        let b = result
            .into_iter()
            .map(|subvec| {
                subvec
                    .into_iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>()
            })
            .collect::<Vec<Vec<String>>>();

        assert_eq!(
            HashSet::<Vec<String>>::from_iter(a.into_iter()),
            HashSet::<Vec<String>>::from_iter(b.into_iter())
        );
    }

    #[test]
    fn example_2() {
        let s = "a".to_string();
        let result = vec![vec!["a"]];

        let a = Solution::partition(s);
        let b = result
            .into_iter()
            .map(|subvec| {
                subvec
                    .into_iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>()
            })
            .collect::<Vec<Vec<String>>>();

        assert_eq!(
            HashSet::<Vec<String>>::from_iter(a.into_iter()),
            HashSet::<Vec<String>>::from_iter(b.into_iter())
        );
    }

    #[test]
    fn example_3() {
        let s = "aa".to_string();
        let result = vec![vec!["a", "a"], vec!["aa"]];

        let a = Solution::partition(s);
        let b = result
            .into_iter()
            .map(|subvec| {
                subvec
                    .into_iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>()
            })
            .collect::<Vec<Vec<String>>>();

        assert_eq!(
            HashSet::<Vec<String>>::from_iter(a.into_iter()),
            HashSet::<Vec<String>>::from_iter(b.into_iter())
        );
    }
}
// @leetup=inject:after_code
