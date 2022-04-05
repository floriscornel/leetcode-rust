// @leetup=custom
// @leetup=info id=11 lang=rust slug=container-with-most-water

/*
* You are given an integer array `height` of length `n`. There are `n` vertical
* lines drawn such that the two endpoints of the `ith` line are `(i, 0)` and `(i,
* height[i])`.
*
* Find two lines that together with the x-axis form a container, such that the
* container contains the most water.
*
* Return *the maximum amount of water a container can store*.
*
* Notice that you may not slant the container.
*
*
* Example 1:
*
* []
* Input: height = [1,8,6,2,5,4,8,3,7]
* Output: 49
* Explanation: The above vertical lines are represented by array [1,8,6,2,5,4,
* 8,3,7]. In this case, the max area of water (blue section) the container can con
* tain is 49.
*
* Example 2:
*
* Input: height = [1,1]
* Output: 1
*
*
* Constraints:
*
* * `n == height.length`
* * `2 <= n <= 105`
* * `0 <= height[i] <= 104`
*
*/
// @leetup=custom
// @leetup=inject:before_code_ex
// before_code_exclude
#![allow(dead_code, unused_variables)]
struct Solution {}
// @leetup=inject:before_code_ex

// @leetup=code
use std::cmp::Ordering;
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let (mut max, mut left, mut right) = (0, 0, height.len() - 1);
        while left < right {
            let current = (right - left) as i32 * i32::min(height[left], height[right]);
            max = i32::max(max, current);
            match height[left].cmp(&height[right]) {
                Ordering::Less => left += 1,
                Ordering::Equal => right -= 1,
                Ordering::Greater => right -= 1,
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
        assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::max_area(vec![1, 1]), 1);
    }
}
// @leetup=inject:after_code
