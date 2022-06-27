// @leetup=custom
// @leetup=info id=1689 lang=rust slug=partitioning-into-minimum-number-of-deci-binary-numbers

/*
* A decimal number is called deci-binary if each of its digits is either `0`
* or `1` without any leading zeros. For example, `101` and `1100` are
* deci-binary, while `112` and `3001` are not.
*
* Given a string `n` that represents a positive decimal integer, return *the
* minimum number of positive deci-binary numbers needed so that they sum
* up to *`n`*.*
*
*
* Example 1:
*
* Input: n = "32"
* Output: 3
* Explanation: 10 + 11 + 11 = 32
*
* Example 2:
*
* Input: n = "82734"
* Output: 8
*
* Example 3:
*
* Input: n = "27346209830709182346"
* Output: 9
*
*
* Constraints:
*
* * `1 <= n.length <= 105`
* * `n` consists of only digits.
* * `n` does not contain any leading zeros and represents a positive integer.
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
    pub fn min_partitions(n: String) -> i32 {
        n.chars()
            .map(|c| c.to_digit(10).unwrap())
            .reduce(u32::max)
            .unwrap() as i32
    }
}
// @leetup=code

// @leetup=inject:after_code
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        assert_eq!(Solution::min_partitions("32".to_owned()), 3);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::min_partitions("82734".to_owned()), 8);
    }

    #[test]
    fn example_3() {
        assert_eq!(
            Solution::min_partitions("27346209830709182346".to_owned()),
            9
        );
    }
}
// @leetup=inject:after_code
