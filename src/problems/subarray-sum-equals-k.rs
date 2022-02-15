// @leetup=custom
// @leetup=info id=560 lang=rust slug=subarray-sum-equals-k

/*
* Given an array of integers `nums` and an integer `k`, return *the total number
* of continuous subarrays whose sum equals to `k`*.
*
*
* Example 1:
*
* Input: nums = [1,1,1], k = 2
* Output: 2
*
* Example 2:
*
* Input: nums = [1,2,3], k = 3
* Output: 2
*
*
* Constraints:
*
* * `1 <= nums.length <= 2 * 104`
* * `-1000 <= nums[i] <= 1000`
* * `-107 <= k <= 107`
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
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let (mut count, mut sum) = (0, 0);
        let mut map: HashMap<i32, i32> = HashMap::with_capacity(nums.len());
        map.insert(0, 1);
        for num in nums {
            sum += num;
            if let Some(val) = map.get(&(sum - k)) {
                count += val;
            }
            *map.entry(sum).or_insert(0) += 1;
        }
        count
    }
}
// @leetup=code

// @leetup=inject:after_code
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        assert_eq!(Solution::subarray_sum(vec![1, 1, 1], 2), 2);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::subarray_sum(vec![1, 2, 3], 3), 2);
    }
}
// @leetup=inject:after_code
