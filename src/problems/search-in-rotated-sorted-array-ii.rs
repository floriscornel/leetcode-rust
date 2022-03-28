// @leetup=custom
// @leetup=info id=81 lang=rust slug=search-in-rotated-sorted-array-ii

/*
* There is an integer array `nums` sorted in non-decreasing order (not necessarily
* with distinct values).
*
* Before being passed to your function, `nums` is rotated at an unknown pivot
* index `k` (`0 <= k < nums.length`) such that the resulting array is `[nums[k],
* nums[k+1], ..., nums[n-1], nums[0], nums[1], ..., nums[k-1]]` (0-indexed).
* For example, `[0,1,2,4,4,4,5,6,6,7]` might be rotated at pivot index `5` and
* become `[4,5,6,6,7,0,1,2,4,4]`.
*
* Given the array `nums` after the rotation and an integer `target`, return
* `true`* if *`target`* is in *`nums`*, or *`false`* if it is not in *`nums`*.*
*
* You must decrease the overall operation steps as much as possible.
*
*
* Example 1:
*
* Input: nums = [2,5,6,0,0,1,2], target = 0
* Output: true
*
* Example 2:
*
* Input: nums = [2,5,6,0,0,1,2], target = 3
* Output: false
*
*
* Constraints:
*
* * `1 <= nums.length <= 5000`
* * `-104 <= nums[i] <= 104`
* * `nums` is guaranteed to be rotated at some pivot.
* * `-104 <= target <= 104`
*
*
* Follow up: This problem is similar to [Search in Rotated Sorted Array][1],
* but `nums` may contain duplicates. Would this affect the runtime complexity?
* How and why?
*
* [1] /problems/search-in-rotated-sorted-array/description/
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
    // Assumes sorted values from nums[i] up to nums[j]
    fn binary_search(nums: &[i32], target: i32, mut i: usize, mut j: usize) -> bool {
        if nums[i] > target || nums[j] < target {
            return false;
        }
        while i < j {
            let pivot = (j + i) / 2;
            if nums[pivot] == target {
                return true;
            } else if nums[pivot] < target {
                i = pivot + 1;
            } else {
                j = pivot;
            }
        }
        nums[i] == target
    }

    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        let (mut i, mut j) = (0, nums.len() - 1);
        //Edge case, only single element in vec, e.g. [1] or [3]
        if nums.len() == 1 {
            return target == nums[0];
        }
        //Edge case, left most or right most element equals target
        if nums[i] == target || nums[j] == target {
            return true;
        }
        //Edge case, k=0
        if nums[i] < nums[j] {
            return Solution::binary_search(&nums, target, i, j);
        }
        // We know len()>1, k!=0, nums[i]!=target, nums[j]!=target
        while i < j - 1 {
            // Skip all the same elements on the left side
            while nums[i] == nums[i + 1] && i < j - 1 {
                i += 1;
            }
            // Skip all the same elements on the right side
            while nums[j] == nums[j - 1] && i < j - 1 {
                j -= 1;
            }
            // Check if there is only one element left
            if i == j {
                return nums[i] == target;
            }
            // Pivot in the middle
            let pivot = (j + i) / 2;
            if nums[pivot] == target {
                return true;
            } else if nums[pivot] > nums[j] {
                i = pivot;
            } else {
                j = pivot;
            }
        }
        // Identified 0..i and j..(nums.len()-1) as the two sorted sequences.
        if target >= nums[0] && target <= nums[i] {
            Solution::binary_search(&nums, target, 0, i)
        } else {
            Solution::binary_search(&nums, target, j, nums.len() - 1)
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
        assert!(!Solution::search(vec![2, 5, 6, 0, 0, 1, 2], 3));
    }

    #[test]
    fn example_2() {
        assert!(!Solution::search(vec![0, 0, 1, 2, 2, 5, 6], 3));
    }

    #[test]
    fn example_3() {
        assert!(!Solution::search(vec![0, 0, 1, 2, 2, 5, 6, 0], 3));
    }

    #[test]
    fn example_4() {
        assert!(Solution::search(
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1],
            2
        ));
    }

    #[test]
    fn example_5() {
        assert!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 5));
    }

    #[test]
    fn example_6() {
        assert!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 1));
    }
}
// @leetup=inject:after_code
