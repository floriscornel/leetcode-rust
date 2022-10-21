// @leetup=custom
// @leetup=info id=219 lang=rust slug=contains-duplicate-ii

/*
* Given an integer array `nums` and an integer `k`, return `true` if there are two
* distinct indices `i` and `j` in the array such that `nums[i] == nums[j]` and
* `abs(i - j) <= k`.
*
*
* Example 1:
*
* Input: nums = [1,2,3,1], k = 3
* Output: true
*
* Example 2:
*
* Input: nums = [1,0,1,1], k = 1
* Output: true
*
* Example 3:
*
* Input: nums = [1,2,3,1,2,3], k = 2
* Output: false
*
*
* Constraints:
*
* * `1 <= nums.length <= 105`
* * `-109 <= nums[i] <= 109`
* * `0 <= k <= 105`
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
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut indices = HashMap::<i32, usize>::with_capacity(nums.len());
        for (idx, num) in nums.iter().enumerate() {
            match indices.get_mut(num) {
                None => {
                    indices.insert(*num, idx);
                }
                Some(previous_index) => {
                    if idx - *previous_index <= k as usize {
                        return true;
                    } else {
                        *previous_index = idx;
                    }
                }
            };
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
        assert!(Solution::contains_nearby_duplicate(vec![1, 2, 3, 1], 3));
    }

    #[test]
    fn example_2() {
        assert!(Solution::contains_nearby_duplicate(vec![1, 0, 1, 1], 1));
    }

    #[test]
    fn example_3() {
        assert!(!Solution::contains_nearby_duplicate(
            vec![1, 2, 3, 1, 2, 3],
            2
        ));
    }
}
// @leetup=inject:after_code
