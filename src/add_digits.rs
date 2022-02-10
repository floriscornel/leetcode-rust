#![allow(dead_code, unused_variables)]
struct Solution {}

/** Add Digits - https://leetcode.com/problems/add-digits/ */

impl Solution {
    pub fn add_digits(num: i32) -> i32 {
        let mut digits = Vec::new();
        let mut pointer = num;
        while pointer > 0 {
            digits.push((pointer % 10) as i32);
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

#[cfg(test)]
mod tests {
    #[test]
    fn add_digits() {
        assert_eq!(super::Solution::add_digits(38), 2);
        assert_eq!(super::Solution::add_digits(0), 0);
    }
}
