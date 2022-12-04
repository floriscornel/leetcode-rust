// @leetup=custom
// @leetup=info id=2256 lang=rust slug=minimum-average-difference

/*
* You are given a 0-indexed integer array `nums` of length `n`.
*
* The average difference of the index `i` is the absolute difference
* between the average of the first `i + 1` elements of `nums` and the average
* of the last `n - i - 1` elements. Both averages should be rounded down
* to the nearest integer.
*
* Return* the index with the minimum average difference*. If there are
* multiple such indices, return the smallest one.
*
* Note:
*
* * The absolute difference of two numbers is the absolute value of their
*   difference.
* * The average of `n` elements is the sum of the `n` elements divided
*   (integer division) by `n`.
* * The average of `0` elements is considered to be `0`.
*
*
* Example 1:
*
* Input: nums = [2,5,3,9,5,3]
* Output: 3
* Explanation:
* - The average difference of index 0 is: |2 / 1 - (5 + 3 + 9 + 5 + 3) / 5| = |2 /
*  1 - 25 / 5| = |2 - 5| = 3.
* - The average difference of index 1 is: |(2 + 5) / 2 - (3 + 9 + 5 + 3) / 4| = |7
*  / 2 - 20 / 4| = |3 - 5| = 2.
* - The average difference of index 2 is: |(2 + 5 + 3) / 3 - (9 + 5 + 3) / 3| = |1
* 0 / 3 - 17 / 3| = |3 - 5| = 2.
* - The average difference of index 3 is: |(2 + 5 + 3 + 9) / 4 - (5 + 3) / 2| = |1
* 9 / 4 - 8 / 2| = |4 - 4| = 0.
* - The average difference of index 4 is: |(2 + 5 + 3 + 9 + 5) / 5 - 3 / 1| = |24
* / 5 - 3 / 1| = |4 - 3| = 1.
* - The average difference of index 5 is: |(2 + 5 + 3 + 9 + 5 + 3) / 6 - 0| = |27
* / 6 - 0| = |4 - 0| = 4.
* The average difference of index 3 is the minimum average difference so return 3.
*
* Example 2:
*
* Input: nums = [0]
* Output: 0
* Explanation:
* The only index is 0 so return 0.
* The average difference of index 0 is: |0 / 1 - 0| = |0 - 0| = 0.
*
*
* Constraints:
*
* * `1 <= nums.length <= 105`
* * `0 <= nums[i] <= 105`
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
    pub fn minimum_average_difference(nums: Vec<i32>) -> i32 {
        let n = nums.len();

        let (mut prefix_sum, mut infix_sum) = (vec![nums[0] as i64], vec![nums[n - 1] as i64]);
        for i in 1..n {
            prefix_sum.push(prefix_sum[i - 1] + nums[i] as i64);
            prefix_sum[i - 1] = f64::floor(prefix_sum[i - 1] as f64 / i as f64) as i64;
            infix_sum.push(infix_sum[i - 1] + nums[n - 1 - i] as i64);
            infix_sum[i - 1] = f64::floor(infix_sum[i - 1] as f64 / i as f64) as i64;
        }
        prefix_sum[n - 1] = f64::floor(prefix_sum[n - 1] as f64 / n as f64) as i64;
        infix_sum[n - 1] = f64::floor(infix_sum[n - 1] as f64 / n as f64) as i64;
        infix_sum.reverse();
        infix_sum.push(0);

        let (mut min_val, mut min_idx) = (i32::MAX, 0);
        for i in 0..n {
            let abs_diff = i64::abs(prefix_sum[i] - infix_sum[i + 1]) as i32;
            if abs_diff < min_val {
                min_val = abs_diff;
                min_idx = i;
            }
        }
        min_idx as i32
    }
}
// @leetup=code

// @leetup=inject:after_code
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::minimum_average_difference(vec![2, 5, 3, 9, 5, 3]),
            3
        )
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::minimum_average_difference(vec![0]), 0)
    }
}
// @leetup=inject:after_code
