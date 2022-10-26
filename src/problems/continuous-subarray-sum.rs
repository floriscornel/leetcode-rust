// @leetup=custom
// @leetup=info id=523 lang=rust slug=continuous-subarray-sum

/*
* Given an integer array `nums` and an integer `k`, return `true` *if *`nums`* has
* a continuous subarray of size at least two whose elements sum up to a
* multiple of* `k`*, or *`false`* otherwise*.
*
* An integer `x` is a multiple of `k` if there exists an integer `n` such that `x
* = n * k`. `0` is always a multiple of `k`.
*
*
* Example 1:
*
* Input: nums = [23,2,4,6,7], k = 6
* Output: true
* Explanation: [2, 4] is a continuous subarray of size 2 whose elements sum up
*  to 6.
*
* Example 2:
*
* Input: nums = [23,2,6,4,7], k = 6
* Output: true
* Explanation: [23, 2, 6, 4, 7] is an continuous subarray of size 5 whose elem
* ents sum up to 42.
* 42 is a multiple of 6 because 42 = 7 * 6 and 7 is an integer.
*
* Example 3:
*
* Input: nums = [23,2,6,4,7], k = 13
* Output: false
*
*
* Constraints:
*
* * `1 <= nums.length <= 105`
* * `0 <= nums[i] <= 109`
* * `0 <= sum(nums[i]) <= 231 - 1`
* * `1 <= k <= 231 - 1`
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
    pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
        let mut map = HashMap::<i32, i32>::new();
        let mut sum_remainder = 0;
        map.insert(0, -1);
        for (current_index, num) in nums.iter().enumerate() {
            sum_remainder += num;
            sum_remainder %= k;
            if let Some(previous_index) = map.get(&sum_remainder) {
                if current_index as i32 - previous_index > 1 {
                    return true;
                }
            } else {
                map.insert(sum_remainder, current_index as i32);
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
        assert!(Solution::check_subarray_sum(vec![23, 2, 4, 6, 7], 6));
    }

    #[test]
    fn example_2() {
        assert!(Solution::check_subarray_sum(vec![23, 2, 6, 4, 7], 6));
    }

    #[test]
    fn example_3() {
        assert!(!Solution::check_subarray_sum(vec![23, 2, 6, 4, 7], 13));
    }

    #[test]
    fn example_4() {
        assert!(!Solution::check_subarray_sum(vec![1, 0], 2));
    }

    #[test]
    fn example_5() {
        assert!(!Solution::check_subarray_sum(vec![1, 2, 12], 6));
    }
}
// @leetup=inject:after_code
