// @leetup=custom
// @leetup=info id=881 lang=rust slug=boats-to-save-people

/*
* You are given an array `people` where `people[i]` is the weight of the `ith`
* person, and an infinite number of boats where each boat can carry a maximum
* weight of `limit`. Each boat carries at most two people at the same time,
* provided the sum of the weight of those people is at most `limit`.
*
* Return *the minimum number of boats to carry every given person*.
*
*
* Example 1:
*
* Input: people = [1,2], limit = 3
* Output: 1
* Explanation: 1 boat (1, 2)
*
* Example 2:
*
* Input: people = [3,2,2,1], limit = 3
* Output: 3
* Explanation: 3 boats (1, 2), (2) and (3)
*
* Example 3:
*
* Input: people = [3,5,3,4], limit = 5
* Output: 4
* Explanation: 4 boats (3), (3), (4), (5)
*
*
* Constraints:
*
* * `1 <= people.length <= 5 * 104`
* * `1 <= people[i] <= limit <= 3 * 104`
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
    pub fn num_rescue_boats(mut people: Vec<i32>, limit: i32) -> i32 {
        people.sort_unstable();
        let mut num_boats = 0;
        let (mut i, mut j) = (0, people.len() - 1);
        while i <= j {
            if people[i] + people[j] <= limit {
                i += 1;
            }
            num_boats += 1;
            if j > 0 {
                j -= 1;
            } else {
                break;
            }
        }
        num_boats
    }
}
// @leetup=code

// @leetup=inject:after_code
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        assert_eq!(Solution::num_rescue_boats(vec![1, 2], 3), 1);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::num_rescue_boats(vec![3, 2, 2, 1], 3), 3);
    }

    #[test]
    fn example_3() {
        assert_eq!(Solution::num_rescue_boats(vec![3, 5, 3, 4], 5), 4);
    }
}
// @leetup=inject:after_code
