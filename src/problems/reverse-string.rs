// @leetup=custom
// @leetup=info id=344 lang=rust slug=reverse-string

/*
* Write a function that reverses a string. The input string is given as an array
* of characters `s`.
*
* You must do this by modifying the input array [in-place][1] with `O(1)` extra
* memory.
*
*
* Example 1:
*
* Input: s = ['h','e','l','l','o']
* Output: ['o','l','l','e','h']
*
* Example 2:
*
* Input: s = ['H','a','n','n','a','h']
* Output: ['h','a','n','n','a','H']
*
*
* Constraints:
*
* * `1 <= s.length <= 105`
* * `s[i]` is a [printable ascii character][1].
*
*   [1] https://en.wikipedia.org/wiki/ASCII#Printable_characters
*
* [1] https://en.wikipedia.org/wiki/In-place_algorithm
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
    pub fn reverse_string(s: &mut Vec<char>) {
        let len = s.len();
        let end = |index: usize| len - index - 1;
        let mid = s.len() / 2;
        for i in 0..mid {
            let tmp = s[end(i)];
            s[end(i)] = s[i];
            s[i] = tmp;
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
        let mut a = vec!['h', 'e', 'l', 'z', 'o'];
        let b = vec!['o', 'z', 'l', 'e', 'h'];
        Solution::reverse_string(&mut a);
        assert_eq!(a, b);
    }

    #[test]
    fn example_2() {
        let mut a = vec!['H', 'a', 'n', 'm', 'a', 'h'];
        let b = vec!['h', 'a', 'm', 'n', 'a', 'H'];
        Solution::reverse_string(&mut a);
        assert_eq!(a, b);
    }

    #[test]
    fn example_3() {
        let mut a = vec!['a'];
        let b = vec!['a'];
        Solution::reverse_string(&mut a);
        assert_eq!(a, b);
    }

    #[test]
    fn example_4() {
        let mut a = vec!['a', 'b'];
        let b = vec!['b', 'a'];
        Solution::reverse_string(&mut a);
        assert_eq!(a, b);
    }

    #[test]
    fn example_5() {
        let mut a = vec!['a', 'b', 'c'];
        let b = vec!['c', 'b', 'a'];
        Solution::reverse_string(&mut a);
        assert_eq!(a, b);
    }
}
// @leetup=inject:after_code
