// @leetup=custom
// @leetup=info id=1323 lang=rust slug=maximum-69-number

/*
* You are given a positive integer `num` consisting only of digits `6` and `9`.
*
* Return *the maximum number you can get by changing at most one digit (*`6`*
* becomes *`9`*, and *`9`* becomes *`6`*)*.
*
*
* Example 1:
*
* Input: num = 9669
* Output: 9969
* Explanation:
* Changing the first digit results in 6669.
* Changing the second digit results in 9969.
* Changing the third digit results in 9699.
* Changing the fourth digit results in 9666.
* The maximum number is 9969.
*
* Example 2:
*
* Input: num = 9996
* Output: 9999
* Explanation: Changing the last digit 6 to 9 results in the maximum number.
*
* Example 3:
*
* Input: num = 9999
* Output: 9999
* Explanation: It is better not to apply any change.
*
*
* Constraints:
*
* * `1 <= num <= 104`
* * `num` consists of only `6` and `9` digits.
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
    pub fn maximum69_number(num: i32) -> i32 {
        let mut modified = false;
        num.to_string()
            .chars()
            .filter_map(|x| {
                let mut y = x.to_digit(10);
                if y == Some(6) && !modified {
                    y = Some(9);
                    modified = true;
                }
                y
            })
            .fold(0, |acc, x| acc * 10 + x) as i32
    }
}
// @leetup=code

// @leetup=inject:after_code
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        assert_eq!(Solution::maximum69_number(9669), 9969);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::maximum69_number(9996), 9999);
    }

    #[test]
    fn example_3() {
        assert_eq!(Solution::maximum69_number(9999), 9999);
    }
}
// @leetup=inject:after_code
