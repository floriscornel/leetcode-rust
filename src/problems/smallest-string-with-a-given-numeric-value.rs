// @leetup=custom
// @leetup=info id=1663 lang=rust slug=smallest-string-with-a-given-numeric-value

/*
* The numeric value of a lowercase character is defined as its position
* `(1-indexed)` in the alphabet, so the numeric value of `a` is `1`, the numeric
* value of `b` is `2`, the numeric value of `c` is `3`, and so on.
*
* The numeric value of a string consisting of lowercase characters is
* defined as the sum of its characters' numeric values. For example, the numeric
* value of the string `"abe"` is equal to `1 + 2 + 5 = 8`.
*
* You are given two integers `n` and `k`. Return *the lexicographically smallest
* string with length equal to `n` and numeric value equal to `k`.*
*
* Note that a string `x` is lexicographically smaller than string `y` if `x` comes
* before `y` in dictionary order, that is, either `x` is a prefix of `y`, or if
* `i` is the first position such that `x[i] != y[i]`, then `x[i]` comes before
* `y[i]` in alphabetic order.
*
*
* Example 1:
*
* Input: n = 3, k = 27
* Output: "aay"
* Explanation: The numeric value of the string is 1 + 1 + 25 = 27, and it is t
* he smallest string with such a value and length equal to 3.
*
* Example 2:
*
* Input: n = 5, k = 73
* Output: "aaszz"
*
*
* Constraints:
*
* * `1 <= n <= 105`
* * `n <= k <= 26 * n`
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
    pub fn get_smallest_string(n: i32, k: i32) -> String {
        let mut values = Vec::with_capacity(n as usize);
        for i in 0..n {
            values.push('a' as u32);
        }
        let mut remainder = (k - n) as u32;
        let mut end_index = n as usize;
        while remainder > 0 {
            end_index -= 1;
            let character_increment = u32::min(remainder, 25);
            values[end_index] += character_increment;
            remainder -= character_increment;
        }
        values.iter().map(|x| char::from_u32(*x).unwrap()).collect()
    }
}
// @leetup=code

// @leetup=inject:after_code
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        assert_eq!(Solution::get_smallest_string(3, 27), "aay".to_string())
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::get_smallest_string(5, 73), "aaszz".to_string())
    }

    #[test]
    fn example_3() {
        assert_eq!(Solution::get_smallest_string(1, 26), "z".to_string())
    }

    #[test]
    fn example_4() {
        assert_eq!(Solution::get_smallest_string(5, 5), "aaaaa".to_string())
    }

    #[test]
    fn example_5() {
        assert_eq!(
            Solution::get_smallest_string(10, 260),
            "zzzzzzzzzz".to_string()
        )
    }

    #[test]
    fn example_6() {
        assert_eq!(Solution::get_smallest_string(1, 1), "a".to_string())
    }
}
// @leetup=inject:after_code
