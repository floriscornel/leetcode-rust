// @leetup=custom
// @leetup=info id=899 lang=rust slug=orderly-queue

/*
* You are given a string `s` and an integer `k`. You can choose one of the first
* `k` letters of `s` and append it at the end of the string..
*
* Return *the lexicographically smallest string you could have after applying the
* mentioned step any number of moves*.
*
*
* Example 1:
*
* Input: s = "cba", k = 1
* Output: "acb"
* Explanation:
* In the first move, we move the 1st character 'c' to the end, obtaining the strin
* g "bac".
* In the second move, we move the 1st character 'b' to the end, obtaining the fina
* l result "acb".
*
* Example 2:
*
* Input: s = "baaca", k = 3
* Output: "aaabc"
* Explanation:
* In the first move, we move the 1st character 'b' to the end, obtaining the strin
* g "aacab".
* In the second move, we move the 3rd character 'c' to the end, obtaining the fina
* l result "aaabc".
*
*
* Constraints:
*
* * `1 <= k <= s.length <= 1000`
* * `s` consist of lowercase English letters.
*
*/
// @leetup=custom
// @leetup=inject:before_code_ex
// before_code_exclude
#![allow(dead_code, unused_variables)]
struct Solution {}
// @leetup=inject:before_code_ex

// @leetup=code
use std::collections::VecDeque;

impl Solution {
    pub fn less(a: &VecDeque<u8>, b: &VecDeque<u8>) -> bool {
        let mut diff = 0;
        for i in 0..a.len() {
            match a[i].cmp(&b[i]) {
                std::cmp::Ordering::Equal => {}
                std::cmp::Ordering::Greater => break,
                std::cmp::Ordering::Less => diff += 1,
            }
        }
        diff > 0
    }
    pub fn orderly_queue(s: String, k: i32) -> String {
        if k > 1 {
            let mut vec = Vec::with_capacity(s.len());
            for c in s.as_bytes().iter().copied() {
                vec.push(c);
            }
            vec.sort_unstable();
            vec.iter().map(|x| *x as char).collect()
        } else {
            let mut sb: VecDeque<u8> = VecDeque::with_capacity(s.len());
            for c in s.as_bytes().iter().copied() {
                sb.push_back(c);
            }
            let mut min_sb = sb.clone();
            for i in 0..=s.len() {
                let first = sb.pop_front().unwrap();
                sb.push_back(first);
                if Solution::less(&sb, &min_sb) {
                    min_sb = sb.clone();
                }
            }
            min_sb.iter().map(|x| *x as char).collect()
        }
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
            Solution::orderly_queue("cba".to_owned(), 1),
            "acb".to_owned()
        )
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::orderly_queue("baaca".to_owned(), 3),
            "aaabc".to_owned()
        )
    }

    #[test]
    fn example_3() {
        assert_eq!(
            Solution::orderly_queue("asgaghwiauoghwaioghwoaihashfa".to_owned(), 1),
            "aasgaghwiauoghwaioghwoaihashf".to_owned()
        )
    }
}
// @leetup=inject:after_code
