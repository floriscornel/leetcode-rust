// @leetup=custom
// @leetup=info id=31 lang=rust slug=next-permutation

/*
* A permutation of an array of integers is an arrangement of its members into
* a sequence or linear order.
*
* * For example, for `arr = [1,2,3]`, the following are considered permutations of
*   `arr`: `[1,2,3]`, `[1,3,2]`, `[3,1,2]`, `[2,3,1]`.
*
* The next permutation of an array of integers is the next lexicographically
* greater permutation of its integer. More formally, if all the permutations of
* the array are sorted in one container according to their lexicographical order,
* then the next permutation of that array is the permutation that follows it
* in the sorted container. If such arrangement is not possible, the array must be
* rearranged as the lowest possible order (i.e., sorted in ascending order).
*
* * For example, the next permutation of `arr = [1,2,3]` is `[1,3,2]`.
* * Similarly, the next permutation of `arr = [2,3,1]` is `[3,1,2]`.
* * While the next permutation of `arr = [3,2,1]` is `[1,2,3]` because `[3,2,1]`
*   does not have a lexicographical larger rearrangement.
*
* Given an array of integers `nums`, *find the next permutation of* `nums`.
*
* The replacement must be [in place][1] and use only constant extra memory.
*
*
* Example 1:
*
* Input: nums = [1,2,3]
* Output: [1,3,2]
*
* Example 2:
*
* Input: nums = [3,2,1]
* Output: [1,2,3]
*
* Example 3:
*
* Input: nums = [1,1,5]
* Output: [1,5,1]
*
*
* Constraints:
*
* * `1 <= nums.length <= 100`
* * `0 <= nums[i] <= 100`
*
* [1] http://en.wikipedia.org/wiki/In-place_algorithm
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
    pub fn next_permutation(nums: &mut [i32]) {
        // Source: https://docs.rs/permutohedron
        // These cases only have 1 permutation each, so we can't do anything.
        if nums.len() < 2 {
            return;
        }

        // Step 1: Identify the longest, rightmost weakly decreasing part of the vector
        let mut i = nums.len() - 1;
        while i > 0 && nums[i - 1] >= nums[i] {
            i -= 1;
        }

        // If that is the entire vector, this is the last-ordered permutation.
        if i == 0 {
            nums.reverse();
            return;
        }

        // Step 2: Find the rightmost element larger than the pivot (i-1)
        let mut j = nums.len() - 1;
        while j >= i && nums[j] <= nums[i - 1] {
            j -= 1;
        }

        // Step 3: Swap that element with the pivot
        nums.swap(j, i - 1);

        // Step 4: Reverse the (previously) weakly decreasing part
        nums[i..].reverse();
    }
}
// @leetup=code

// @leetup=inject:after_code
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let mut input = vec![1, 2, 3];
        Solution::next_permutation(&mut input);
        assert_eq!(input, vec![1, 3, 2]);
    }

    #[test]
    fn example_2() {
        let mut input = vec![3, 2, 1];
        Solution::next_permutation(&mut input);
        assert_eq!(input, vec![1, 2, 3]);
    }

    #[test]
    fn example_3() {
        let mut input = vec![1, 1, 5];
        Solution::next_permutation(&mut input);
        assert_eq!(input, vec![1, 5, 1]);
    }
}
// @leetup=inject:after_code
