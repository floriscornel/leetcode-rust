// @leetup=custom
// @leetup=info id=258 lang=rust slug=add-digits

/*
* Given an integer `num`, repeatedly add all its digits until the result has only
* one digit, and return it.
*
*
* Example 1:
*
* Input: num = 38
* Output: 2
* Explanation: The process is
* 38 --> 3 + 8 --> 11
* 11 --> 1 + 1 --> 2
* Since 2 has only one digit, return it.
*
* Example 2:
*
* Input: num = 0
* Output: 0
*
*
* Constraints:
*
* * `0 <= num <= 231 - 1`
*
*
* Follow up: Could you do it without any loop/recursion in `O(1)` runtime?
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
    pub fn add_digits(num: i32) -> i32 {
        let mut digits = Vec::new();
        let mut pointer = num;
        while pointer > 0 {
            digits.push(pointer % 10);
            pointer /= 10;
        }
        let result = digits.iter().sum();
        if result < 10 {
            result
        } else {
            Solution::add_digits(result)
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
        assert_eq!(Solution::add_digits(38), 2);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::add_digits(0), 0);
    }
}
// @leetup=inject:after_code
