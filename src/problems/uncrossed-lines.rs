// @leetup=custom
// @leetup=info id=1035 lang=rust slug=uncrossed-lines

/*
* You are given two integer arrays `nums1` and `nums2`. We write the integers of
* `nums1` and `nums2` (in the order they are given) on two separate horizontal
* lines.
*
* We may draw connecting lines: a straight line connecting two numbers `nums1[i]`
* and `nums2[j]` such that:
*
* * `nums1[i] == nums2[j]`, and
* * the line we draw does not intersect any other connecting (non-horizontal)
*   line.
*
* Note that a connecting line cannot intersect even at the endpoints (i.e., each
* number can only belong to one connecting line).
*
* Return *the maximum number of connecting lines we can draw in this way*.
*
*
* Example 1:
*
* []
* Input: nums1 = [1,4,2], nums2 = [1,2,4]
* Output: 2
* Explanation: We can draw 2 uncrossed lines as in the diagram.
* We cannot draw 3 uncrossed lines, because the line from nums1[1] = 4 to nums2[2]
*  = 4 will intersect the line from nums1[2]=2 to nums2[1]=2.
*
* Example 2:
*
* Input: nums1 = [2,5,1,2,5], nums2 = [10,5,2,1,5,2]
* Output: 3
*
* Example 3:
*
* Input: nums1 = [1,3,7,1,7,5], nums2 = [1,9,2,5,1]
* Output: 2
*
*
* Constraints:
*
* * `1 <= nums1.length, nums2.length <= 500`
* * `1 <= nums1[i], nums2[j] <= 2000`
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
    pub fn max_uncrossed_lines(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut max_lines = vec![0; nums2.len() + 1];
        for a in nums1 {
            let mut count = 0;
            for (pos, &b) in nums2.iter().enumerate() {
                let current = max_lines[pos + 1];
                if a == b {
                    max_lines[pos + 1] = count + 1;
                } else {
                    max_lines[pos + 1] = max_lines[pos + 1].max(max_lines[pos]);
                }
                count = current;
            }
        }
        max_lines[nums2.len()]
    }
}
// @leetup=code

// @leetup=inject:after_code
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let nums1 = vec![1, 4, 2];
        let nums2 = vec![1, 2, 4];
        let expected = 2;
        assert_eq!(Solution::max_uncrossed_lines(nums1, nums2), expected);
    }

    #[test]
    fn example_2() {
        let nums1 = vec![2, 5, 1, 2, 5];
        let nums2 = vec![10, 5, 2, 1, 5, 2];
        let expected = 3;
        assert_eq!(Solution::max_uncrossed_lines(nums1, nums2), expected);
    }

    #[test]
    fn example_3() {
        let nums1 = vec![1, 3, 7, 1, 7, 5];
        let nums2 = vec![1, 9, 2, 5, 1];
        let expected = 2;
        assert_eq!(Solution::max_uncrossed_lines(nums1, nums2), expected);
    }

    #[test]
    fn example_4() {
        let nums1 = vec![1, 2, 3, 4, 9, 5, 6, 7, 8];
        let nums2 = vec![5, 6, 7, 8, 9, 1, 2, 3, 4];
        let expected = 4;
        assert_eq!(Solution::max_uncrossed_lines(nums1, nums2), expected);
    }
}
// @leetup=inject:after_code
