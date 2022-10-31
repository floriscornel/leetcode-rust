// @leetup=custom
// @leetup=info id=22 lang=rust slug=generate-parentheses

/*
* Given `n` pairs of parentheses, write a function to *generate all combinations
* of well-formed parentheses*.
*
*
* Example 1:
*
* Input: n = 3
* Output: ["((()))","(()())","(())()","()(())","()()()"]
*
* Example 2:
*
* Input: n = 1
* Output: ["()"]
*
*
* Constraints:
*
* * `1 <= n <= 8`
*
*/
// @leetup=custom
// @leetup=inject:before_code_ex
// before_code_exclude
#![allow(dead_code, unused_variables)]

struct Solution {}
// @leetup=inject:before_code_ex

// @leetup=code
use std::collections::HashSet;
impl Solution {
    pub fn generate_parenthesis(mut n: i32) -> Vec<String> {
        let mut vec = vec!["".to_owned()];
        while n > 0 {
            let mut new_vec = HashSet::new();
            for str in &vec {
                for i in 0..=str.len() {
                    let mut new_string = str.to_owned();
                    new_string.insert_str(i, "()");
                    new_vec.insert(new_string);
                }
            }
            vec = new_vec.into_iter().collect();
            n -= 1;
        }
        vec
    }
}
// @leetup=code

// @leetup=inject:after_code
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let mut res = Solution::generate_parenthesis(3);
        let mut expected = vec![
            "((()))".to_owned(),
            "(()())".to_owned(),
            "(())()".to_owned(),
            "()(())".to_owned(),
            "()()()".to_owned(),
        ];
        res.sort_unstable();
        expected.sort_unstable();
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::generate_parenthesis(1), vec!["()".to_owned(),]);
    }
}
// @leetup=inject:after_code
