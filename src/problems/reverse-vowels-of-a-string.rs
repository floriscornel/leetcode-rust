// @leetup=custom
// @leetup=info id=345 lang=rust slug=reverse-vowels-of-a-string

/*
* Given a string `s`, reverse only all the vowels in the string and return it.
*
* The vowels are `'a'`, `'e'`, `'i'`, `'o'`, and `'u'`, and they can appear in
* both lower and upper cases, more than once.
*
*
* Example 1:
*
* Input: s = "hello"
* Output: "holle"
*
* Example 2:
*
* Input: s = "leetcode"
* Output: "leotcede"
*
*
* Constraints:
*
* * `1 <= s.length <= 3 * 105`
* * `s` consist of printable ASCII characters.
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
    pub fn reverse_vowels(s: String) -> String {
        let mut vowels = s.chars().fold(Vec::new(), |mut acc, x| match x {
            'a' | 'i' | 'u' | 'e' | 'o' | 'A' | 'I' | 'U' | 'E' | 'O' => {
                acc.push(x);
                acc
            }
            _ => acc,
        });

        s.chars()
            .map(|x| match x {
                'a' | 'i' | 'u' | 'e' | 'o' | 'A' | 'I' | 'U' | 'E' | 'O' => vowels.pop().unwrap(),
                _ => x,
            })
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
            Solution::reverse_vowels("hello".to_owned()),
            "holle".to_owned()
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::reverse_vowels("leetcode".to_owned()),
            "leotcede".to_owned()
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(Solution::reverse_vowels("aA".to_owned()), "Aa".to_owned());
    }
}
// @leetup=inject:after_code
