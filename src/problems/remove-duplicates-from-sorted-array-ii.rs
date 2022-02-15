// @leetup=custom
// @leetup=info id=80 lang=rust slug=remove-duplicates-from-sorted-array-ii

/*
* Given an integer array `nums` sorted in non-decreasing order, remove some
* duplicates [in-place][1] such that each unique element appears at most
* twice. The relative order of the elements should be kept the same.
*
* Since it is impossible to change the length of the array in some languages, you
* must instead have the result be placed in the first part of the array
* `nums`. More formally, if there are `k` elements after removing the duplicates,
* then the first `k` elements of `nums` should hold the final result. It does not
* matter what you leave beyond the first `k` elements.
*
* Return `k`* after placing the final result in the first *`k`* slots of *`nums`.
*
* Do not allocate extra space for another array. You must do this by
* modifying the input array [in-place][2] with O(1) extra memory.
*
* Custom Judge:
*
* The judge will test your solution with the following code:
*
* int[] nums = [...]; // Input array
* int[] expectedNums = [...]; // The expected answer with correct length
* int k = removeDuplicates(nums); // Calls your implementation
* assert k == expectedNums.length;
* for (int i = 0; i < k; i++) {
*     assert nums[i] == expectedNums[i];
* }
*
* If all assertions pass, then your solution will be accepted.
*
*
* Example 1:
*
* Input: nums = [1,1,1,2,2,3]
* Output: 5, nums = [1,1,2,2,3,_]
* Explanation: Your function should return k = 5, with the first five elements
*  of nums being 1, 1, 2, 2 and 3 respectively.
* It does not matter what you leave beyond the returned k (hence they are undersco
* res).
*
* Example 2:
*
* Input: nums = [0,0,1,1,1,1,2,3,3]
* Output: 7, nums = [0,0,1,1,2,3,3,_,_]
* Explanation: Your function should return k = 7, with the first seven element
* s of nums being 0, 0, 1, 1, 2, 3 and 3 respectively.
* It does not matter what you leave beyond the returned k (hence they are undersco
* res).
*
*
* Constraints:
*
* * `1 <= nums.length <= 3 * 104`
* * `-104 <= nums[i] <= 104`
* * `nums` is sorted in non-decreasing order.
*
* [1] https://en.wikipedia.org/wiki/In-place_algorithm
* [2] https://en.wikipedia.org/wiki/In-place_algorithm
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
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() <= 2 {
            return nums.len() as i32;
        }

        let mut write = 2;
        for read in 2..nums.len() {
            if nums[write - 2] != nums[read] {
                nums[write] = nums[read];
                write += 1;
            }
        }
        write as i32
    }
}
// @leetup=code

// @leetup=inject:after_code
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let mut nums = vec![1, 1, 1, 2, 2, 3];
        assert_eq!(Solution::remove_duplicates(&mut nums), 5);
    }

    #[test]
    fn example_2() {
        let mut nums = vec![0, 0, 1, 1, 1, 1, 2, 3, 3];
        assert_eq!(Solution::remove_duplicates(&mut nums), 7);
    }
}
// @leetup=inject:after_code
