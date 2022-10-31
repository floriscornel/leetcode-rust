// @leetup=custom
// @leetup=info id=49 lang=rust slug=group-anagrams

/*
* Given an array of strings `strs`, group the anagrams together. You can
* return the answer in any order.
*
* An Anagram is a word or phrase formed by rearranging the letters of a
* different word or phrase, typically using all the original letters exactly once.
*
*
* Example 1:
*
* Input: strs = ["eat","tea","tan","ate","nat","bat"]
* Output: [["bat"],["nat","tan"],["ate","eat","tea"]]
*
* Example 2:
*
* Input: strs = [""]
* Output: [[""]]
*
* Example 3:
*
* Input: strs = ["a"]
* Output: [["a"]]
*
*
* Constraints:
*
* * `1 <= strs.length <= 104`
* * `0 <= strs[i].length <= 100`
* * `strs[i]` consists of lowercase English letters.
*
*/
// @leetup=custom
// @leetup=inject:before_code_ex
// before_code_exclude
#![allow(dead_code, unused_variables)]

struct Solution {}
// @leetup=inject:before_code_ex

// @leetup=code
use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map = HashMap::<[u8; 26], Vec<String>>::new();
        for str in strs {
            let mut hist = [0; 26];
            for char in str.as_bytes() {
                hist[(*char - b'a') as usize] += 1;
            }
            map.entry(hist).or_insert(Vec::new()).push(str);
        }
        map.into_values().collect()
    }
}
// @leetup=code

// @leetup=inject:after_code
#[cfg(test)]
mod tests {
    use super::Solution;

    // Note: Order of hashmap changes.
    #[test]
    fn example_1() {
        let mut res = Solution::group_anagrams(vec![
            "eat".to_owned(),
            "tea".to_owned(),
            "tan".to_owned(),
            "ate".to_owned(),
            "nat".to_owned(),
            "bat".to_owned(),
        ]);
        let mut expected = vec![
            vec!["tan".to_owned(), "nat".to_owned()],
            vec!["eat".to_owned(), "tea".to_owned(), "ate".to_owned()],
            vec!["bat".to_owned()],
        ];
        res.sort_unstable();
        expected.sort_unstable();
        assert_eq!(res, expected)
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::group_anagrams(vec!["a".to_owned(),]),
            vec![vec!["a".to_owned()],]
        )
    }

    #[test]
    fn example_3() {
        assert_eq!(
            Solution::group_anagrams(vec!["".to_owned(),]),
            vec![vec!["".to_owned()],]
        )
    }
}
// @leetup=inject:after_code
