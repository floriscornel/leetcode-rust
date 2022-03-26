// @leetup=custom
// @leetup=info id=704 lang=rust slug=binary-search

/*
* Given an array of integers `nums` which is sorted in ascending order, and an
* integer `target`, write a function to search `target` in `nums`. If `target`
* exists, then return its index. Otherwise, return `-1`.
*
* You must write an algorithm with `O(log n)` runtime complexity.
*
*
* Example 1:
*
* Input: nums = [-1,0,3,5,9,12], target = 9
* Output: 4
* Explanation: 9 exists in nums and its index is 4
*
* Example 2:
*
* Input: nums = [-1,0,3,5,9,12], target = 2
* Output: -1
* Explanation: 2 does not exist in nums so return -1
*
*
* Constraints:
*
* * `1 <= nums.length <= 104`
* * `-104 < nums[i], target < 104`
* * All the integers in `nums` are unique.
* * `nums` is sorted in ascending order.
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
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let (mut head, mut tail) = (0, nums.len() - 1);
        while head < tail {
            let middle = (head + tail) / 2;
            match nums[middle].cmp(&target) {
                std::cmp::Ordering::Less => head = middle + 1,
                std::cmp::Ordering::Equal => return middle as i32,
                std::cmp::Ordering::Greater => tail = middle,
            }
        }
        if nums[head] == target {
            head as i32
        } else {
            -1
        }
    }
}
// @leetup=code

// @leetup=inject:after_code
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        assert_eq!(Solution::search(vec![-1, 0, 3, 5, 9, 12], 9), 4);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::search(vec![-1, 0, 3, 5, 9, 12], 2), -1);
    }
}
// @leetup=inject:after_code
