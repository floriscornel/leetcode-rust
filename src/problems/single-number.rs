// @leetup=custom
// @leetup=info id=136 lang=rust slug=single-number

/*
* Given a non-empty array of integers `nums`, every element appears *twice*
* except for one. Find that single one.
*
* You must implement a solution with a linear runtime complexity and use only
* constant extra space.
*
*
* Example 1:
*
* Input: nums = [2,2,1]
* Output: 1
*
* Example 2:
*
* Input: nums = [4,1,2,1,2]
* Output: 4
*
* Example 3:
*
* Input: nums = [1]
* Output: 1
*
*
* Constraints:
*
* * `1 <= nums.length <= 3 * 104`
* * `-3 * 104 <= nums[i] <= 3 * 104`
* * Each element in the array appears twice except for one element which appears
*   only once.
*
*/
// @leetup=custom
// @leetup=inject:before_code_ex
// before_code_exclude
#![allow(dead_code, unused_variables)]
struct Solution {}
// @leetup=inject:before_code_ex

// @leetup=code

// @leetup=inject:before_code
// before_code
// @leetup=inject:before_code
use std::collections::HashSet;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut set: HashSet<i32> = HashSet::with_capacity(nums.len());
        for num in nums {
            if set.contains(&num) {
                set.remove(&num);
            } else {
                set.insert(num);
            }
        }
        *set.iter().next().unwrap()
    }
}
// @leetup=code

// @leetup=inject:after_code
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        assert_eq!(Solution::single_number(vec![2, 2, 1]), 1);
        assert_eq!(Solution::single_number(vec![4, 1, 2, 1, 2]), 4);
        assert_eq!(Solution::single_number(vec![1]), 1);
    }
}
// @leetup=inject:after_code
