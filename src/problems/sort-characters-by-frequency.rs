// @leetup=custom
// @leetup=info id=451 lang=rust slug=sort-characters-by-frequency

/*
* Given a string `s`, sort it in decreasing order based on the frequency
* of the characters. The frequency of a character is the number of times it
* appears in the string.
*
* Return *the sorted string*. If there are multiple answers, return *any of them*.
*
*
* Example 1:
*
* Input: s = "tree"
* Output: "eert"
* Explanation: 'e' appears twice while 'r' and 't' both appear once.
* So 'e' must appear before both 'r' and 't'. Therefore "eetr" is also a valid ans
* wer.
*
* Example 2:
*
* Input: s = "cccaaa"
* Output: "aaaccc"
* Explanation: Both 'c' and 'a' appear three times, so both "cccaaa" and "aaac
* cc" are valid answers.
* Note that "cacaca" is incorrect, as the same characters must be together.
*
* Example 3:
*
* Input: s = "Aabb"
* Output: "bbAa"
* Explanation: "bbaA" is also a valid answer, but "Aabb" is incorrect.
* Note that 'A' and 'a' are treated as two different characters.
*
*
* Constraints:
*
* * `1 <= s.length <= 5 * 105`
* * `s` consists of uppercase and lowercase English letters and digits.
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
    pub fn frequency_sort(s: String) -> String {
        let mut hist = s
            .into_bytes()
            .into_iter()
            .fold(HashMap::<char, usize>::new(), |mut map, char| {
                let key = char as char;
                map.entry(key).and_modify(|c| *c += 1).or_insert(1);
                map
            })
            .into_iter()
            .map(|(char, count)| (count, char))
            .collect::<Vec<_>>();

        hist.sort_unstable();

        hist.into_iter()
            .rev()
            .flat_map(|(count, char)| vec![char; count])
            .collect()
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
            Solution::frequency_sort("tree".to_owned()),
            "eetr".to_owned()
        );
        assert_eq!(
            Solution::frequency_sort("cccaaa".to_owned()),
            "cccaaa".to_owned()
        );
        assert_eq!(
            Solution::frequency_sort("Aabb".to_owned()),
            "bbaA".to_owned()
        );
    }
}
// @leetup=inject:after_code
