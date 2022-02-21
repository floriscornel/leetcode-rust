// @leetup=custom
// @leetup=info id=402 lang=rust slug=remove-k-digits

/*
* Given string num representing a non-negative integer `num`, and an integer `k`,
* return *the smallest possible integer after removing* `k` *digits from* `num`.
*
*
* Example 1:
*
* Input: num = "1432219", k = 3
* Output: "1219"
* Explanation: Remove the three digits 4, 3, and 2 to form the new number 1219
*  which is the smallest.
*
* Example 2:
*
* Input: num = "10200", k = 1
* Output: "200"
* Explanation: Remove the leading 1 and the number is 200. Note that the outpu
* t must not contain leading zeroes.
*
* Example 3:
*
* Input: num = "10", k = 2
* Output: "0"
* Explanation: Remove all the digits from the number and it is left with nothi
* ng which is 0.
*
*
* Constraints:
*
* * `1 <= k <= num.length <= 105`
* * `num` consists of only digits.
* * `num` does not have any leading zeros except for the zero itself.
*
*/
// @leetup=custom
// @leetup=inject:before_code_ex
// before_code_exclude
#![allow(dead_code, unused_variables, clippy::comparison_chain)]
struct Solution {}
// @leetup=inject:before_code_ex

// @leetup=code

impl Solution {
    //  https://leetcode.com/problems/remove-k-digits/discuss/629831/Rust-stack-efficient-malloc-O(n)-time-and-space.-(0ms-faster-than-100.)
    pub fn remove_kdigits(num: String, k: i32) -> String {
        let mut k = k as usize;
        let mut stack = String::with_capacity(num.len());
        for digit in num.chars() {
            while k > 0 && !stack.is_empty() && digit < stack.chars().last().unwrap() {
                stack.pop();
                k -= 1;
            }
            if stack.is_empty() && digit == '0' {
                continue;
            }
            stack.push(digit);
        }
        for _ in 0..k {
            stack.pop();
        }
        if stack.is_empty() {
            String::from("0")
        } else {
            stack
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
            Solution::remove_kdigits("1432219".to_string(), 3),
            "1219".to_string()
        )
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::remove_kdigits("10200".to_string(), 1),
            "200".to_string()
        )
    }

    #[test]
    fn example_3() {
        assert_eq!(
            Solution::remove_kdigits("10".to_string(), 2),
            "0".to_string()
        )
    }

    #[test]
    fn example_4() {
        assert_eq!(
            Solution::remove_kdigits("1234567890".to_string(), 10),
            "0".to_string()
        )
    }
}
// @leetup=inject:after_code
