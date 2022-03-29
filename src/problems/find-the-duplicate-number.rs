// @leetup=custom
// @leetup=info id=287 lang=rust slug=find-the-duplicate-number

/*
* Given an array of integers `nums` containing `n + 1` integers where each integer
* is in the range `[1, n]` inclusive.
*
* There is only one repeated number in `nums`, return *this repeated number*.
*
* You must solve the problem without modifying the array `nums` and uses only
* constant extra space.
*
*
* Example 1:
*
* Input: nums = [1,3,4,2,2]
* Output: 2
*
* Example 2:
*
* Input: nums = [3,1,3,4,2]
* Output: 3
*
*
* Constraints:
*
* * `1 <= n <= 105`
* * `nums.length == n + 1`
* * `1 <= nums[i] <= n`
* * All the integers in `nums` appear only once except for precisely one
*   integer which appears two or more times.
*
*
* Follow up:
*
* * How can we prove that at least one duplicate number must exist in `nums`?
* * Can you solve the problem in linear runtime complexity?
*
*/
// @leetup=custom
// @leetup=inject:before_code_ex
// before_code_exclude
#![allow(dead_code, unused_variables)]
struct Solution {}
// @leetup=inject:before_code_ex

// @leetup=code

// Implementation of The Tortoise and the Hare (Floyd’s Algorithm)
// https://medium.com/@tuvo1106/the-tortoise-and-the-hare-floyds-algorithm-87badf5f7d41
impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        // Helper functions
        // First element of the nums array (as usize)
        let first = || nums[0] as usize;
        // Next element in the list
        let next = |x: usize| nums[x] as usize;

        // Define our two nodes
        let mut p1: usize = first();
        let mut p2: usize = first();

        loop {
            // Move through the list at different speeds
            p1 = next(p1);
            p2 = next(next(p2));

            // If both reach the same element
            if p1 == p2 {
                // Reset p1
                p1 = first();
                while p1 != p2 {
                    // Move at same speed
                    p1 = next(p1);
                    p2 = next(p2);
                }
                return p2 as i32;
            }
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
        assert_eq!(Solution::find_duplicate(vec![1, 3, 4, 2, 2]), 2);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::find_duplicate(vec![3, 1, 3, 4, 2]), 3);
    }

    #[test]
    fn example_3() {
        assert_eq!(Solution::find_duplicate(vec![3, 1, 3, 4, 3]), 3);
    }

    #[test]
    fn example_4() {
        assert_eq!(Solution::find_duplicate(vec![2, 2, 2, 2, 2, 2]), 2);
    }

    #[test]
    fn example_5() {
        assert_eq!(
            Solution::find_duplicate(vec![2, 5, 9, 6, 9, 3, 8, 9, 7, 1]),
            9
        );
    }
}
// @leetup=inject:after_code
