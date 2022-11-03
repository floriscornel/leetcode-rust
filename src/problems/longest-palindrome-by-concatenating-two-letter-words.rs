// @leetup=custom
// @leetup=info id=2131 lang=rust slug=longest-palindrome-by-concatenating-two-letter-words

/*
* You are given an array of strings `words`. Each element of `words` consists of
* two lowercase English letters.
*
* Create the longest possible palindrome by selecting some elements from
* `words` and concatenating them in any order. Each element can be selected
* at most once.
*
* Return *the length of the longest palindrome that you can create*. If it is
* impossible to create any palindrome, return `0`.
*
* A palindrome is a string that reads the same forward and backward.
*
*
* Example 1:
*
* Input: words = ["lc","cl","gg"]
* Output: 6
* Explanation: One longest palindrome is "lc" + "gg" + "cl" = "lcggcl", of len
* gth 6.
* Note that "clgglc" is another longest palindrome that can be created.
*
* Example 2:
*
* Input: words = ["ab","ty","yt","lc","cl","ab"]
* Output: 8
* Explanation: One longest palindrome is "ty" + "lc" + "cl" + "yt" = "tylcclyt
* ", of length 8.
* Note that "lcyttycl" is another longest palindrome that can be created.
*
* Example 3:
*
* Input: words = ["cc","ll","xx"]
* Output: 2
* Explanation: One longest palindrome is "cc", of length 2.
* Note that "ll" is another longest palindrome that can be created, and so is "xx"
* .
*
*
* Constraints:
*
* * `1 <= words.length <= 105`
* * `words[i].length == 2`
* * `words[i]` consists of lowercase English letters.
*
*/
// @leetup=custom
// @leetup=inject:before_code_ex
// before_code_exclude
#![allow(dead_code, unused_variables)]

use std::collections::HashMap;
struct Solution {}
// @leetup=inject:before_code_ex

// @leetup=code

impl Solution {
    pub fn longest_palindrome(words: Vec<String>) -> i32 {
        let mut middle_was_found = false;
        let mut map = HashMap::<[u8; 2], usize>::new();
        for word in words {
            let mut x = [0; 2];
            x.copy_from_slice(&word.as_bytes()[0..2]);
            map.entry(x).and_modify(|count| *count += 1).or_insert(1);
        }
        map.iter()
            .map(|(&[a, b], &c1)| {
                let k1 = [a, b];
                let k2 = [b, a];
                if k1 == k2 {
                    if c1 % 2 == 1 {
                        middle_was_found = true;
                    }
                    (c1 / 2) * 4
                } else {
                    2 * c1.min(*map.get(&k2).unwrap_or(&0))
                }
            })
            .sum::<usize>() as i32
            + if middle_was_found { 2 } else { 0 }
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
            Solution::longest_palindrome(vec!["lc".to_owned(), "cl".to_owned(), "gg".to_owned()]),
            6
        )
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::longest_palindrome(vec![
                "ab".to_owned(),
                "ty".to_owned(),
                "yt".to_owned(),
                "lc".to_owned(),
                "cl".to_owned(),
                "ab".to_owned()
            ]),
            8
        )
    }

    #[test]
    fn example_3() {
        assert_eq!(
            Solution::longest_palindrome(vec!["cc".to_owned(), "ll".to_owned(), "xx".to_owned()]),
            2
        )
    }
}
// @leetup=inject:after_code
