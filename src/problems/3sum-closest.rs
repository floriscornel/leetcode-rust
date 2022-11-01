// @leetup=custom
// @leetup=info id=16 lang=rust slug=3sum-closest

/*
* Given an integer array `nums` of length `n` and an integer `target`, find three
* integers in `nums` such that the sum is closest to `target`.
*
* Return *the sum of the three integers*.
*
* You may assume that each input would have exactly one solution.
*
*
* Example 1:
*
* Input: nums = [-1,2,1,-4], target = 1
* Output: 2
* Explanation: The sum that is closest to the target is 2. (-1 + 2 + 1 = 2).
*
* Example 2:
*
* Input: nums = [0,0,0], target = 1
* Output: 0
* Explanation: The sum that is closest to the target is 0. (0 + 0 + 0 = 0).
*
*
* Constraints:
*
* * `3 <= nums.length <= 500`
* * `-1000 <= nums[i] <= 1000`
* * `-104 <= target <= 104`
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
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let (n, mut closest, mut dist) = (nums.len(), i32::MAX, i32::MAX);
        for i in 0..n {
            for j in (i + 1)..n {
                for k in (j + 1)..n {
                    if i32::abs(target - (nums[i] + nums[j] + nums[k])) < dist {
                        dist = i32::abs(target - (nums[i] + nums[j] + nums[k]));
                        closest = nums[i] + nums[j] + nums[k];
                    }
                }
            }
        }
        closest
    }
}
// @leetup=code

// @leetup=inject:after_code
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        assert_eq!(Solution::three_sum_closest(vec![-1, 2, 1, -4], 1), 2)
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::three_sum_closest(vec![0, 0, 0], 1), 0)
    }
}
// @leetup=inject:after_code
