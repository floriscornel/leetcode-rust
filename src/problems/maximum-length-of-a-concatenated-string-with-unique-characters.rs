// @leetup=custom
// @leetup=info id=1239 lang=rust slug=maximum-length-of-a-concatenated-string-with-unique-characters

/*
* You are given an array of strings `arr`. A string `s` is formed by the
* concatenation of a subsequence of `arr` that has unique characters.
*
* Return *the maximum possible length* of `s`.
*
* A subsequence is an array that can be derived from another array by deleting
* some or no elements without changing the order of the remaining elements.
*
*
* Example 1:
*
* Input: arr = ["un","iq","ue"]
* Output: 4
* Explanation: All the valid concatenations are:
* - ""
* - "un"
* - "iq"
* - "ue"
* - "uniq" ("un" + "iq")
* - "ique" ("iq" + "ue")
* Maximum length is 4.
*
* Example 2:
*
* Input: arr = ["cha","r","act","ers"]
* Output: 6
* Explanation: Possible longest valid concatenations are "chaers" ("cha" + "er
* s") and "acters" ("act" + "ers").
*
* Example 3:
*
* Input: arr = ["abcdefghijklmnopqrstuvwxyz"]
* Output: 26
* Explanation: The only string in arr has all 26 characters.
*
*
* Constraints:
*
* * `1 <= arr.length <= 16`
* * `1 <= arr[i].length <= 26`
* * `arr[i]` contains only lowercase English letters.
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
    pub fn max_size(arr: &[[bool; 26]], min_index: usize, current: [bool; 26]) -> i32 {
        let size = |hist: &[bool; 26]| hist.iter().filter(|x| **x).count() as i32;
        let mut max_size = size(&current);
        for next_index in min_index..arr.len() {
            let (mut new, mut unique) = (current, true);
            for i in 0..26 {
                if current[i] && arr[next_index][i] {
                    unique = false;
                    break;
                }
                new[i] = current[i] || arr[next_index][i];
            }
            if unique {
                max_size = max_size.max(Solution::max_size(arr, next_index + 1, new));
            }
        }
        max_size
    }

    pub fn max_length(arr: Vec<String>) -> i32 {
        let hists: Vec<[bool; 26]> = arr
            .iter()
            .map(|string| {
                let mut hist = [false; 26];
                for char in string.chars() {
                    let idx = char as usize - b'a' as usize;
                    if hist[idx] {
                        hist = [false; 26];
                        break;
                    }
                    hist[char as usize - b'a' as usize] = true;
                }
                hist
            })
            .collect();
        Solution::max_size(&hists, 0, [false; 26])
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
            Solution::max_length(vec!["un".to_owned(), "iq".to_owned(), "ue".to_owned()]),
            4
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::max_length(vec![
                "cha".to_owned(),
                "r".to_owned(),
                "act".to_owned(),
                "ers".to_owned()
            ]),
            6
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            Solution::max_length(vec!["abcdefghijklmnopqrstuvwxyz".to_owned()]),
            26
        );
    }
}
// @leetup=inject:after_code
