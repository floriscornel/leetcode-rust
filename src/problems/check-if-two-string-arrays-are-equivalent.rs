// @leetup=custom
// @leetup=info id=1662 lang=rust slug=check-if-two-string-arrays-are-equivalent

/*
* Given two string arrays `word1` and `word2`, return* *`true`* if the two arrays
* represent the same string, and *`false`* otherwise.*
*
* A string is represented by an array if the array elements concatenated in
* order forms the string.
*
*
* Example 1:
*
* Input: word1 = ["ab", "c"], word2 = ["a", "bc"]
* Output: true
* Explanation:
* word1 represents string "ab" + "c" -> "abc"
* word2 represents string "a" + "bc" -> "abc"
* The strings are the same, so return true.
*
* Example 2:
*
* Input: word1 = ["a", "cb"], word2 = ["ab", "c"]
* Output: false
*
* Example 3:
*
* Input: word1  = ["abc", "d", "defg"], word2 = ["abcddefg"]
* Output: true
*
*
* Constraints:
*
* * `1 <= word1.length, word2.length <= 103`
* * `1 <= word1[i].length, word2[i].length <= 103`
* * `1 <= sum(word1[i].length), sum(word2[i].length) <= 103`
* * `word1[i]` and `word2[i]` consist of lowercase letters.
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
    pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
        let (mut x1, mut x2, mut y1, mut y2) = (0, 0, 0, 0);
        while x1 < word1.len() && x2 < word2.len() {
            if word1[x1].as_bytes()[y1] != word2[x2].as_bytes()[y2] {
                return false;
            }
            y1 += 1;
            y2 += 1;
            if word1[x1].len() == y1 {
                x1 += 1;
                y1 = 0;
            }
            if word2[x2].len() == y2 {
                x2 += 1;
                y2 = 0;
            }
        }
        x1 == word1.len() && x2 == word2.len()
    }
}
// @leetup=code

// @leetup=inject:after_code
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        assert!(Solution::array_strings_are_equal(
            vec!["ab".to_owned(), "c".to_owned()],
            vec!["a".to_owned(), "bc".to_owned()]
        ));
    }

    #[test]
    fn example_2() {
        assert!(!Solution::array_strings_are_equal(
            vec!["a".to_owned(), "cb".to_owned()],
            vec!["ab".to_owned(), "c".to_owned()]
        ));
    }

    #[test]
    fn example_3() {
        assert!(Solution::array_strings_are_equal(
            vec!["abc".to_owned(), "d".to_owned(), "defg".to_owned()],
            vec!["abcddefg".to_owned()]
        ));
    }

    #[test]
    fn example_4() {
        assert!(!Solution::array_strings_are_equal(
            vec!["abc".to_owned(), "d".to_owned(), "defg".to_owned()],
            vec!["abcddef".to_owned()]
        ));
    }
}
// @leetup=inject:after_code
