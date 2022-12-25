// @leetup=custom
// @leetup=info id=2389 lang=rust slug=longest-subsequence-with-limited-sum

/*
* You are given an integer array `nums` of length `n`, and an integer array
* `queries` of length `m`.
*
* Return *an array *`answer`* of length *`m`* where *`answer[i]`* is the
* maximum size of a subsequence that you can take from *`nums`* such that
* the sum of its elements is less than or equal to *`queries[i]`.
*
* A subsequence is an array that can be derived from another array by deleting
* some or no elements without changing the order of the remaining elements.
*
*
* Example 1:
*
* Input: nums = [4,5,2,1], queries = [3,10,21]
* Output: [2,3,4]
* Explanation: We answer the queries as follows:
* - The subsequence [2,1] has a sum less than or equal to 3. It can be proven that
*  2 is the maximum size of such a subsequence, so answer[0] = 2.
* - The subsequence [4,5,1] has a sum less than or equal to 10. It can be proven t
* hat 3 is the maximum size of such a subsequence, so answer[1] = 3.
* - The subsequence [4,5,2,1] has a sum less than or equal to 21. It can be proven
*  that 4 is the maximum size of such a subsequence, so answer[2] = 4.
*
* Example 2:
*
* Input: nums = [2,3,4,5], queries = [1]
* Output: [0]
* Explanation: The empty subsequence is the only subsequence that has a sum le
* ss than or equal to 1, so answer[0] = 0.
*
*
* Constraints:
*
* * `n == nums.length`
* * `m == queries.length`
* * `1 <= n, m <= 1000`
* * `1 <= nums[i], queries[i] <= 106`
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
    pub fn answer_queries(nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
        let (n, m) = (nums.len(), queries.len());
        queries
            .into_iter()
            .map(|q| {
                let mut dp = vec![0; q as usize + 1];
                for &num in &nums {
                    if num > q {
                        continue;
                    }
                    for i in (num..=q).rev() {
                        dp[i as usize] = dp[i as usize].max(dp[(i - num) as usize] + 1);
                    }
                }
                dp[q as usize]
            })
            .collect()
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
            Solution::answer_queries(vec![4, 5, 2, 1], vec![3, 10, 21]),
            vec![2, 3, 4]
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::answer_queries(vec![2, 3, 4, 5], vec![1]), vec![0]);
    }
}
// @leetup=inject:after_code
