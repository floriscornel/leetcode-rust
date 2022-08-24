// @leetup=custom
// @leetup=info id=326 lang=rust slug=power-of-three

/*
* Given an integer `n`, return *`true` if it is a power of three. Otherwise,
* return `false`*.
*
* An integer `n` is a power of three, if there exists an integer `x` such that `n
* == 3x`.
*
*
* Example 1:
*
* Input: n = 27
* Output: true
*
* Example 2:
*
* Input: n = 0
* Output: false
*
* Example 3:
*
* Input: n = 9
* Output: true
*
*
* Constraints:
*
* * `-231 <= n <= 231 - 1`
*
*
* Follow up: Could you solve it without loops/recursion?
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
    pub fn is_power_of_three(n: i32) -> bool {
        const LARGER_POWER: i32 = 3_i32.pow(19);
        n > 0 && LARGER_POWER % n == 0
    }

    pub fn is_power_of_three_loop(n: i32) -> bool {
        if n <= 0 {
            return false;
        }
        for exponent in 1..20 {
            let found_power_of_three = 3_i32.pow(exponent);
            if found_power_of_three == n {
                return true;
            }
            if found_power_of_three > n {
                return false;
            }
        }
        false
    }

    pub fn is_power_of_three_lookup(n: i32) -> bool {
        if n <= 0 {
            return false;
        }
        const CORRECT_ANSWERS: [i32; 19] = [
            3, 9, 27, 81, 243, 729, 2187, 6561, 19683, 59049, 177147, 531441, 1594323, 4782969,
            14348907, 43046721, 129140163, 387420489, 1162261467,
        ];
        CORRECT_ANSWERS.contains(&n)
    }
}
// @leetup=code

// @leetup=inject:after_code
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        assert!(Solution::is_power_of_three(27));
        assert!(!Solution::is_power_of_three(0));
        assert!(!Solution::is_power_of_three(-3));
        assert!(Solution::is_power_of_three(9));
        assert!(!Solution::is_power_of_three(8));
        assert!(!Solution::is_power_of_three(10));
        assert!(Solution::is_power_of_three(387420489));
        assert!(Solution::is_power_of_three(1162261467));
        assert!(!Solution::is_power_of_three(1162261466));
        assert!(!Solution::is_power_of_three(1162261468));
        assert!(!Solution::is_power_of_three(i32::MAX));
        assert!(!Solution::is_power_of_three(i32::MIN));
        assert!(Solution::is_power_of_three(243));
    }

    #[test]
    fn example_2() {
        assert!(Solution::is_power_of_three_loop(27));
        assert!(!Solution::is_power_of_three_loop(0));
        assert!(Solution::is_power_of_three_loop(9));
        assert!(!Solution::is_power_of_three_loop(8));
        assert!(!Solution::is_power_of_three_loop(10));
        assert!(Solution::is_power_of_three_loop(387420489));
        assert!(Solution::is_power_of_three_loop(1162261467));
        assert!(!Solution::is_power_of_three_loop(1162261466));
        assert!(!Solution::is_power_of_three_loop(1162261468));
        assert!(!Solution::is_power_of_three_loop(i32::MAX));
        assert!(!Solution::is_power_of_three_loop(i32::MIN));
        assert!(Solution::is_power_of_three_loop(243));
    }

    #[test]
    fn example_3() {
        assert!(Solution::is_power_of_three_lookup(27));
        assert!(!Solution::is_power_of_three_lookup(0));
        assert!(Solution::is_power_of_three_lookup(9));
        assert!(!Solution::is_power_of_three_lookup(8));
        assert!(!Solution::is_power_of_three_lookup(10));
        assert!(Solution::is_power_of_three_lookup(387420489));
        assert!(Solution::is_power_of_three_lookup(1162261467));
        assert!(!Solution::is_power_of_three_lookup(1162261466));
        assert!(!Solution::is_power_of_three_lookup(1162261468));
        assert!(!Solution::is_power_of_three_lookup(i32::MAX));
        assert!(!Solution::is_power_of_three_lookup(i32::MIN));
        assert!(Solution::is_power_of_three_lookup(243));
    }
}
// @leetup=inject:after_code
