// @leetup=custom
// @leetup=info id=169 lang=rust slug=majority-element

/*
* Given an array `nums` of size `n`, return *the majority element*.
*
* The majority element is the element that appears more than `⌊n / 2⌋` times. You
* may assume that the majority element always exists in the array.
*
*
* Example 1:
*
* Input: nums = [3,2,3]
* Output: 3
*
* Example 2:
*
* Input: nums = [2,2,1,1,1,2,2]
* Output: 2
*
*
* Constraints:
*
* * `n == nums.length`
* * `1 <= n <= 5 * 104`
* * `-231 <= nums[i] <= 231 - 1`
*
*
* Follow-up: Could you solve the problem in linear time and in `O(1)` space?
*
*/
// @leetup=custom
// @leetup=inject:before_code_ex
// before_code_exclude
#![allow(dead_code, unused_variables)]
struct Solution {}
// @leetup=inject:before_code_ex

// @leetup=code
use std::collections::HashMap;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut found: HashMap<i32, i32> = HashMap::new();
        let target = nums.len() as i32 / 2;
        for num in nums {
            let count = found.entry(num).or_insert(0);
            *count += 1;
            if *count > target {
                return num;
            }
        }
        panic!("not found.")
    }
}
// @leetup=code

// @leetup=inject:after_code
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        assert_eq!(Solution::majority_element(vec![3, 2, 3]), 3)
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::majority_element(vec![2, 2, 1, 1, 1, 2, 2]), 2)
    }
}
// @leetup=inject:after_code
