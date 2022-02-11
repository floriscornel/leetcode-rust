#![allow(dead_code, unused_variables)]
struct Solution {}

/** Permutation in String - https://leetcode.com/problems/permutation-in-string/ */

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

#[cfg(test)]
mod tests {
    #[test]
    fn example_problem() {
        assert_eq!(
            super::Solution::check_inclusion("a".to_string(), "ab".to_string()),
            true
        );
        assert_eq!(
            super::Solution::check_inclusion("aa".to_string(), "aa".to_string()),
            true
        );
        assert_eq!(
            super::Solution::check_inclusion("a".to_string(), "a".to_string()),
            true
        );
        assert_eq!(
            super::Solution::check_inclusion("a".to_string(), "b".to_string()),
            false
        );
        assert_eq!(
            super::Solution::check_inclusion("ab".to_string(), "ba".to_string()),
            true
        );
        assert_eq!(
            super::Solution::check_inclusion("ab".to_string(), "eidbaooo".to_string()),
            true
        );
        assert_eq!(
            super::Solution::check_inclusion("ab".to_string(), "eidboaoo".to_string()),
            false
        );
        assert_eq!(
            super::Solution::check_inclusion("fed".to_string(), "abcdefghi".to_string()),
            true
        );
        assert_eq!(
            super::Solution::check_inclusion(
                "abcdefghijklmnopqrstuvwxyz".to_string(),
                "abcdefghijklmnopqrstuvwxyaaaazazzzbcdefghijklmnopqrstuvwxyz".to_string()
            ),
            false
        );
    }
}
