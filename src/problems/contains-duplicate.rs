// @leetup=custom
// @leetup=info id=217 lang=rust slug=contains-duplicate

/*
* Given an integer array `nums`, return `true` if any value appears at least
* twice in the array, and return `false` if every element is distinct.
*
*
* Example 1:
*
* Input: nums = [1,2,3,1]
* Output: true
*
* Example 2:
*
* Input: nums = [1,2,3,4]
* Output: false
*
* Example 3:
*
* Input: nums = [1,1,1,3,3,4,3,2,4,2]
* Output: true
*
*
* Constraints:
*
* * `1 <= nums.length <= 105`
* * `-109 <= nums[i] <= 109`
*
*/
// @leetup=custom
// @leetup=inject:before_code_ex
// before_code_exclude
#![allow(dead_code, unused_variables)]

struct Solution {}
// @leetup=inject:before_code_ex

// @leetup=code
use std::collections::HashSet;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut set = HashSet::with_capacity(nums.len());
        for num in nums {
            if !set.insert(num) {
                return true;
            }
        }
        false
    }
}
// @leetup=code

// @leetup=inject:after_code
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        assert!(Solution::contains_duplicate(vec![1, 2, 3, 1]))
    }

    #[test]
    fn example_2() {
        assert!(!Solution::contains_duplicate(vec![1, 2, 3, 4]))
    }

    #[test]
    fn example_3() {
        assert!(Solution::contains_duplicate(vec![
            1, 1, 1, 3, 3, 4, 3, 2, 4, 2
        ]))
    }
}
// @leetup=inject:after_code
