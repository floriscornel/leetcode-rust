// @leetup=custom
// @leetup=info id=42 lang=rust slug=trapping-rain-water

/*
* Given `n` non-negative integers representing an elevation map where the width of
* each bar is `1`, compute how much water it can trap after raining.
*
*
* Example 1:
*
* Input: height = [0,1,0,2,1,0,1,3,2,1,2,1]
* Output: 6
* Explanation: The above elevation map (black section) is represented by array
*  [0,1,0,2,1,0,1,3,2,1,2,1]. In this case, 6 units of rain water (blue section) a
* re being trapped.
*
* Example 2:
*
* Input: height = [4,2,0,3,2,5]
* Output: 9
*
*
* Constraints:
*
* * `n == height.length`
* * `1 <= n <= 2 * 104`
* * `0 <= height[i] <= 105`
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
    pub fn trap(height: Vec<i32>) -> i32 {
        let n = height.len();
        let mut drained = Vec::<i32>::with_capacity(n);
        for i in 0..n {
            drained.push(i32::MAX);
        }
        let (mut left_dh, mut right_dh) = (0, 0);
        for i in 0..n {
            let j = n - 1 - i;
            left_dh = left_dh.max(height[i]);
            right_dh = right_dh.max(height[j]);
            drained[i] = drained[i].min(left_dh);
            drained[j] = drained[j].min(right_dh);
        }
        let mut diff = 0;
        for i in 0..n {
            diff += drained[i] - height[i];
        }
        diff
    }
}
// @leetup=code

// @leetup=inject:after_code
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        assert_eq!(Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::trap(vec![4, 2, 0, 3, 2, 5]), 9);
    }
}
// @leetup=inject:after_code
