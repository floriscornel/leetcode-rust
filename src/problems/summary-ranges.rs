// @leetup=custom
// @leetup=info id=228 lang=rust slug=summary-ranges

/*
* You are given a sorted unique integer array `nums`.
*
* Return *the smallest sorted list of ranges that cover all the numbers in
* the array exactly*. That is, each element of `nums` is covered by exactly one
* of the ranges, and there is no integer `x` such that `x` is in one of the ranges
* but not in `nums`.
*
* Each range `[a,b]` in the list should be output as:
*
* * `"a->b"` if `a != b`
* * `"a"` if `a == b`
*
*
* Example 1:
*
* Input: nums = [0,1,2,4,5,7]
* Output: ["0->2","4->5","7"]
* Explanation: The ranges are:
* [0,2] --> "0->2"
* [4,5] --> "4->5"
* [7,7] --> "7"
*
* Example 2:
*
* Input: nums = [0,2,3,4,6,8,9]
* Output: ["0","2->4","6","8->9"]
* Explanation: The ranges are:
* [0,0] --> "0"
* [2,4] --> "2->4"
* [6,6] --> "6"
* [8,9] --> "8->9"
*
*
* Constraints:
*
* * `0 <= nums.length <= 20`
* * `-231 <= nums[i] <= 231 - 1`
* * All the values of `nums` are unique.
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
use std::cmp::Ordering;

impl Solution {
    fn range_to_string(begin: i32, end: i32) -> String {
        match begin.cmp(&end) {
            Ordering::Less => format!("{begin}->{end}"),
            Ordering::Equal => format!("{begin}"),
            Ordering::Greater => format!("{end}->{begin}"),
        }
    }

    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let mut result = Vec::new();
        let mut i = 0;
        while i < nums.len() {
            let (begin, mut end) = (nums[i], nums[i]);
            while i + 1 < nums.len() && nums[i + 1] == end + 1 {
                i += 1;
                end = nums[i];
            }
            result.push(Solution::range_to_string(begin, end));
            i += 1;
        }
        result
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
            Solution::summary_ranges(vec![0, 1, 2, 4, 5, 7]),
            vec!["0->2", "4->5", "7"]
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::summary_ranges(vec![0, 2, 3, 4, 6, 8, 9]),
            vec!["0", "2->4", "6", "8->9"]
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            Solution::summary_ranges(Vec::<i32>::new()),
            Vec::<String>::new()
        );
    }

    #[test]
    fn example_4() {
        assert_eq!(Solution::summary_ranges(vec![0]), vec!["0"]);
    }
}
// @leetup=inject:after_code
