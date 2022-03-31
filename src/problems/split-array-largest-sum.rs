// @leetup=custom
// @leetup=info id=410 lang=rust slug=split-array-largest-sum

/*
* Given an array `nums` which consists of non-negative integers and an integer
* `m`, you can split the array into `m` non-empty continuous subarrays.
*
* Write an algorithm to minimize the largest sum among these `m` subarrays.
*
*
* Example 1:
*
* Input: nums = [7,2,5,10,8], m = 2
* Output: 18
* Explanation:
* There are four ways to split nums into two subarrays.
* The best way is to split it into [7,2,5] and [10,8],
* where the largest sum among the two subarrays is only 18.
*
* Example 2:
*
* Input: nums = [1,2,3,4,5], m = 2
* Output: 9
*
* Example 3:
*
* Input: nums = [1,4,4], m = 3
* Output: 4
*
*
* Constraints:
*
* * `1 <= nums.length <= 1000`
* * `0 <= nums[i] <= 106`
* * `1 <= m <= min(50, nums.length)`
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
    pub fn split_array(nums: Vec<i32>, m: i32) -> i32 {
        let mut high = nums.iter().sum::<i32>();
        let mut low = *nums.iter().max().unwrap();
        while low < high {
            let mid = low + (high - low) / 2;
            let (mut sum, mut counter) = (0, 1);
            for num in &nums {
                if sum + *num <= mid {
                    sum += *num;
                } else {
                    sum = *num;
                    counter += 1;
                }
            }
            if counter > m {
                low = mid + 1;
            } else {
                high = mid;
            }
        }
        low
    }
}
// @leetup=code

// @leetup=inject:after_code
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        assert_eq!(Solution::split_array(vec![7, 2, 5, 10, 8], 2), 18)
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::split_array(vec![1, 2, 3, 4, 5], 2), 9)
    }

    #[test]
    fn example_3() {
        assert_eq!(Solution::split_array(vec![1, 4, 4], 3), 4)
    }
}
// @leetup=inject:after_code
