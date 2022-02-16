// @leetup=custom
// @leetup=info id=525 lang=rust slug=contiguous-array

/*
* Given a binary array `nums`, return *the maximum length of a contiguous subarray
* with an equal number of *`0`* and *`1`.
*
*
* Example 1:
*
* Input: nums = [0,1]
* Output: 2
* Explanation: [0, 1] is the longest contiguous subarray with an equal number
* of 0 and 1.
*
* Example 2:
*
* Input: nums = [0,1,0]
* Output: 2
* Explanation: [0, 1] (or [1, 0]) is a longest contiguous subarray with equal
* number of 0 and 1.
*
*
* Constraints:
*
* * `1 <= nums.length <= 105`
* * `nums[i]` is either `0` or `1`.
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
    pub fn find_max_length(nums: Vec<i32>) -> i32 {
        let (mut sum, mut max): (i32, i32) = (0, 0);
        let mut map = HashMap::<i32, i32>::new();

        map.insert(0, -1);
        for (index, num) in nums.iter().enumerate() {
            match *num {
                0 => sum -= 1,
                1 => sum += 1,
                _ => panic!("incorrect input"),
            }
            if let Some(first) = map.get(&sum) {
                max = i32::max(max, index as i32 - first);
            } else {
                map.insert(sum, index as i32);
            }
        }
        max
    }
}
// @leetup=code

// @leetup=inject:after_code
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        assert_eq!(Solution::find_max_length(vec![0, 1]), 2);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::find_max_length(vec![0, 1, 0]), 2);
    }

    #[test]
    fn example_3() {
        assert_eq!(
            Solution::find_max_length(vec![1, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1]),
            8
        );
    }

    #[test]
    fn example_4() {
        assert_eq!(
            Solution::find_max_length(vec![0, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0, 0]),
            10
        );
    }

    #[test]
    fn example_5() {
        assert_eq!(Solution::find_max_length(vec![0, 0, 1, 1]), 4);
    }
}
// @leetup=inject:after_code
