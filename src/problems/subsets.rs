// @leetup=custom
// @leetup=info id=78 lang=rust slug=subsets

/*
* Given an integer array `nums` of unique elements, return *all possible
* subsets (the power set)*.
*
* The solution set must not contain duplicate subsets. Return the solution in
* any order.
*
*
* Example 1:
*
* Input: nums = [1,2,3]
* Output: [[],[1],[2],[1,2],[3],[1,3],[2,3],[1,2,3]]
*
* Example 2:
*
* Input: nums = [0]
* Output: [[],[0]]
*
*
* Constraints:
*
* * `1 <= nums.length <= 10`
* * `-10 <= nums[i] <= 10`
* * All the numbers of `nums` are unique.
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
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.iter().fold(vec![vec![]], |mut p, x| {
            let i = p.clone().into_iter().map(|mut s| {
                s.push(*x);
                s
            });
            p.extend(i);
            p
        })
    }
}
// @leetup=code

// @leetup=inject:after_code
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        let input = vec![1, 2, 3];
        let output = vec![
            vec![],
            vec![1],
            vec![2],
            vec![1, 2],
            vec![3],
            vec![1, 3],
            vec![2, 3],
            vec![1, 2, 3],
        ];
        assert_eq!(Solution::subsets(input), output);
    }

    #[test]
    fn example2() {
        let input = vec![0];
        let output = vec![vec![], vec![0]];
        assert_eq!(Solution::subsets(input), output);
    }
}
// @leetup=inject:after_code
