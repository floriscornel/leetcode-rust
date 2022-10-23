// @leetup=custom
// @leetup=info id=645 lang=rust slug=set-mismatch

/*
* You have a set of integers `s`, which originally contains all the numbers from
* `1` to `n`. Unfortunately, due to some error, one of the numbers in `s` got
* duplicated to another number in the set, which results in repetition of one
* number and loss of another number.
*
* You are given an integer array `nums` representing the data status of this set
* after the error.
*
* Find the number that occurs twice and the number that is missing and return
* *them in the form of an array*.
*
*
* Example 1:
*
* Input: nums = [1,2,2,4]
* Output: [2,3]
*
* Example 2:
*
* Input: nums = [1,1]
* Output: [1,2]
*
*
* Constraints:
*
* * `2 <= nums.length <= 104`
* * `1 <= nums[i] <= 104`
*
*/
// @leetup=custom
// @leetup=inject:before_code_ex
// before_code_exclude
#![allow(dead_code, unused_variables)]

struct Solution {}
// @leetup=inject:before_code_ex

// @leetup=code
use std::collections::HashSet;

impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut set = HashSet::<i32>::with_capacity(n);
        let (mut duplicated_number, mut found_sum) = (0, 0);
        for num in nums {
            if set.contains(&num) {
                duplicated_number = num;
            } else {
                set.insert(num);
            }
            found_sum += num;
        }
        found_sum -= duplicated_number;
        let expected_sum = n * (n + 1) / 2;
        let missing_number = i32::abs(expected_sum as i32 - found_sum);
        vec![duplicated_number, missing_number]
    }
}
// @leetup=code

// @leetup=inject:after_code
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        assert_eq!(Solution::find_error_nums(vec![1, 2, 2, 4]), vec![2, 3]);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::find_error_nums(vec![1, 1]), vec![1, 2]);
    }

    #[test]
    fn example_3() {
        assert_eq!(Solution::find_error_nums(vec![4, 3, 2, 2]), vec![2, 1]);
    }

    #[test]
    fn example_4() {
        assert_eq!(
            Solution::find_error_nums(vec![1, 2, 3, 4, 5, 6, 6, 7, 8, 10]),
            vec![6, 9]
        );
    }
}
// @leetup=inject:after_code
