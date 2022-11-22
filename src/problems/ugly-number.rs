// @leetup=custom
// @leetup=info id=263 lang=rust slug=ugly-number

/*
* An ugly number is a positive integer whose prime factors are limited to `2`,
* `3`, and `5`.
*
* Given an integer `n`, return `true` *if* `n` *is an ugly number*.
*
*
* Example 1:
*
* Input: n = 6
* Output: true
* Explanation: 6 = 2 × 3
*
* Example 2:
*
* Input: n = 1
* Output: true
* Explanation: 1 has no prime factors, therefore all of its prime factors are
* limited to 2, 3, and 5.
*
* Example 3:
*
* Input: n = 14
* Output: false
* Explanation: 14 is not ugly since it includes the prime factor 7.
*
*
* Constraints:
*
* * `-231 <= n <= 231 - 1`
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
    pub fn is_ugly(n: i32) -> bool {
        if n <= 0 {
            false
        } else if n % 5 == 0 {
            Solution::is_ugly(n / 5)
        } else if n % 2 == 0 {
            Solution::is_ugly(n / 2)
        } else if n % 3 == 0 {
            Solution::is_ugly(n / 3)
        } else {
            n == 1
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
        assert!(Solution::is_ugly(6));
        assert!(Solution::is_ugly(1));
        assert!(!Solution::is_ugly(14));
    }
}
// @leetup=inject:after_code
