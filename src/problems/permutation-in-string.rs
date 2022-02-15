// @leetup=custom
// @leetup=info id=567 lang=rust slug=permutation-in-string

/*
* Given two strings `s1` and `s2`, return `true`* if *`s2`* contains a permutation
* of *`s1`*, or *`false`* otherwise*.
*
* In other words, return `true` if one of `s1`'s permutations is the substring of
* `s2`.
*
*
* Example 1:
*
* Input: s1 = "ab", s2 = "eidbaooo"
* Output: true
* Explanation: s2 contains one permutation of s1 ("ba").
*
* Example 2:
*
* Input: s1 = "ab", s2 = "eidboaoo"
* Output: false
*
*
* Constraints:
*
* * `1 <= s1.length, s2.length <= 104`
* * `s1` and `s2` consist of lowercase English letters.
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
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let mut permutation_histogram: Vec<i32> = Vec::with_capacity(26);
        let mut rolling_histogram: Vec<i32> = Vec::with_capacity(26);
        for i in 0..26 {
            permutation_histogram.push(0);
            rolling_histogram.push(0);
        }
        for (_idx, char) in s1.chars().enumerate() {
            permutation_histogram[((char as u8) - 97) as usize] += 1;
        }
        for (idx, char) in s2.chars().enumerate() {
            rolling_histogram[((char as u8) - 97) as usize] += 1;
            if idx >= s1.len() {
                rolling_histogram[(s2.as_bytes()[idx - s1.len()] - 97) as usize] -= 1;
            }
            if rolling_histogram == permutation_histogram {
                return true;
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
        assert!(Solution::check_inclusion(
            "ab".to_string(),
            "eidbaooo".to_string()
        ));
    }

    #[test]
    fn example_2() {
        assert!(!Solution::check_inclusion(
            "ab".to_string(),
            "eidboaoo".to_string()
        ));
    }
}
// @leetup=inject:after_code
