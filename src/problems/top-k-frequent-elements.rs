// @leetup=custom
// @leetup=info id=347 lang=rust slug=top-k-frequent-elements

/*
* Given an integer array `nums` and an integer `k`, return *the* `k` *most
* frequent elements*. You may return the answer in any order.
*
*
* Example 1:
*
* Input: nums = [1,1,1,2,2,3], k = 2
* Output: [1,2]
*
* Example 2:
*
* Input: nums = [1], k = 1
* Output: [1]
*
*
* Constraints:
*
* * `1 <= nums.length <= 105`
* * `k` is in the range `[1, the number of unique elements in the array]`.
* * It is guaranteed that the answer is unique.
*
*
* Follow up: Your algorithm's time complexity must be better than `O(n log
* n)`, where n is the array's size.
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
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut hist = HashMap::<i32, i32>::new();
        for num in nums {
            *hist.entry(num).or_insert(0) += 1;
        }
        let mut res: Vec<(i32, i32)> = hist.iter().map(|(a, b)| (*a, *b)).collect();
        res.sort_unstable_by(|(a, b), (x, y)| (y).cmp(b));
        res.iter().take(k as usize).map(|(a, b)| *a).collect()
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
            Solution::top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2),
            vec![1, 2]
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::top_k_frequent(vec![1], 1), vec![1]);
    }
}
// @leetup=inject:after_code
