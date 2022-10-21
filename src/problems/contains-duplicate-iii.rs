// @leetup=custom
// @leetup=info id=220 lang=rust slug=contains-duplicate-iii

/*
* You are given an integer array `nums` and two integers `indexDiff` and
* `valueDiff`.
*
* Find a pair of indices `(i, j)` such that:
*
* * `i != j`,
* * `abs(i - j) <= indexDiff`.
* * `abs(nums[i] - nums[j]) <= valueDiff`, and
*
* Return `true`* if such pair exists or *`false`* otherwise*.
*
*
* Example 1:
*
* Input: nums = [1,2,3,1], indexDiff = 3, valueDiff = 0
* Output: true
* Explanation: We can choose (i, j) = (0, 3).
* We satisfy the three conditions:
* i != j --> 0 != 3
* abs(i - j) <= indexDiff --> abs(0 - 3) <= 3
* abs(nums[i] - nums[j]) <= valueDiff --> abs(1 - 1) <= 0
*
* Example 2:
*
* Input: nums = [1,5,9,1,5,9], indexDiff = 2, valueDiff = 3
* Output: false
* Explanation: After trying all the possible pairs (i, j), we cannot satisfy t
* he three conditions, so we return false.
*
*
* Constraints:
*
* * `2 <= nums.length <= 105`
* * `-109 <= nums[i] <= 109`
* * `1 <= indexDiff <= nums.length`
* * `0 <= valueDiff <= 109`
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
    pub fn contains_nearby_almost_duplicate(
        nums: Vec<i32>,
        index_diff: i32,
        value_diff: i32,
    ) -> bool {
        // O(n^2) but fast enough to be accepted by leetcode.
        for i in 0..nums.len() {
            for j in (i + 1)..(nums.len().min(i + index_diff as usize + 1)) {
                if i32::abs(nums[i] - nums[j]) <= value_diff {
                    return true;
                }
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
        assert!(Solution::contains_nearby_almost_duplicate(
            vec![1, 2, 3, 1],
            3,
            0
        ));
    }

    #[test]
    fn example_2() {
        assert!(!Solution::contains_nearby_almost_duplicate(
            vec![1, 5, 9, 1, 5, 9],
            2,
            3
        ));
    }
}
// @leetup=inject:after_code
