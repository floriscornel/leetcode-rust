// @leetup=custom
// @leetup=info id=907 lang=rust slug=sum-of-subarray-minimums

/*
* Given an array of integers arr, find the sum of `min(b)`, where `b` ranges over
* every (contiguous) subarray of `arr`. Since the answer may be large, return the
* answer modulo `109 + 7`.
*
*
* Example 1:
*
* Input: arr = [3,1,2,4]
* Output: 17
* Explanation:
* Subarrays are [3], [1], [2], [4], [3,1], [1,2], [2,4], [3,1,2], [1,2,4], [3,1,2,
* 4].
* Minimums are 3, 1, 2, 4, 1, 1, 2, 1, 1, 1.
* Sum is 17.
*
* Example 2:
*
* Input: arr = [11,81,94,43,3]
* Output: 444
*
*
* Constraints:
*
* * `1 <= arr.length <= 3 * 104`
* * `1 <= arr[i] <= 3 * 104`
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
    pub fn sum_subarray_mins(arr: Vec<i32>) -> i32 {
        const MOD: usize = 1_000_000_007;
        let (mut dp, mut stack) = (vec![0; arr.len()], vec![]);
        for index in 0..arr.len() {
            while !stack.is_empty() && arr[*stack.last().unwrap()] >= arr[index] {
                stack.pop();
            }
            if let Some(&top_index) = stack.last() {
                dp[index] = dp[top_index] + (index - top_index) * arr[index] as usize;
            } else {
                dp[index] = (index + 1) * arr[index] as usize;
            }
            stack.push(index);
        }
        (dp.iter().fold(0, |acc, v| (acc + v) % MOD) % MOD) as _
    }
}
// @leetup=code

// @leetup=inject:after_code
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        assert_eq!(Solution::sum_subarray_mins(vec![3, 1, 2, 4]), 17);
        assert_eq!(Solution::sum_subarray_mins(vec![11, 81, 94, 43, 3]), 444);
    }
}
// @leetup=inject:after_code
