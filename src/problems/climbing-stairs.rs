// @leetup=custom
// @leetup=info id=70 lang=rust slug=climbing-stairs

/*
* You are climbing a staircase. It takes `n` steps to reach the top.
*
* Each time you can either climb `1` or `2` steps. In how many distinct ways can
* you climb to the top?
*
*
* Example 1:
*
* Input: n = 2
* Output: 2
* Explanation: There are two ways to climb to the top.
* 1. 1 step + 1 step
* 2. 2 steps
*
* Example 2:
*
* Input: n = 3
* Output: 3
* Explanation: There are three ways to climb to the top.
* 1. 1 step + 1 step + 1 step
* 2. 1 step + 2 steps
* 3. 2 steps + 1 step
*
*
* Constraints:
*
* * `1 <= n <= 45`
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
    pub fn climb_stairs(n: i32) -> i32 {
        (1..n).fold((1, 1), |(a, b), _| (b, a + b)).1
    }
}
// @leetup=code

// @leetup=inject:after_code
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        assert_eq!(Solution::climb_stairs(2), 2);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::climb_stairs(3), 3);
    }

    #[test]
    fn example_3() {
        assert_eq!(Solution::climb_stairs(33), 5702887);
    }
}
// @leetup=inject:after_code
