// @leetup=custom
// @leetup=info id=2244 lang=rust slug=minimum-rounds-to-complete-all-tasks

/*
* You are given a 0-indexed integer array `tasks`, where `tasks[i]` represents
* the difficulty level of a task. In each round, you can complete either 2 or 3
* tasks of the same difficulty level.
*
* Return *the minimum rounds required to complete all the tasks, or *`-1`* if
* it is not possible to complete all the tasks.*
*
*
* Example 1:
*
* Input: tasks = [2,2,3,3,2,4,4,4,4,4]
* Output: 4
* Explanation: To complete all the tasks, a possible plan is:
* - In the first round, you complete 3 tasks of difficulty level 2.
* - In the second round, you complete 2 tasks of difficulty level 3.
* - In the third round, you complete 3 tasks of difficulty level 4.
* - In the fourth round, you complete 2 tasks of difficulty level 4.
* It can be shown that all the tasks cannot be completed in fewer than 4 rounds, s
* o the answer is 4.
*
* Example 2:
*
* Input: tasks = [2,3,3]
* Output: -1
* Explanation: There is only 1 task of difficulty level 2, but in each round,
* you can only complete either 2 or 3 tasks of the same difficulty level. Hence, y
* ou cannot complete all the tasks, and the answer is -1.
*
*
* Constraints:
*
* * `1 <= tasks.length <= 105`
* * `1 <= tasks[i] <= 109`
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
    pub fn minimum_rounds(tasks: Vec<i32>) -> i32 {
        let mut counts = HashMap::<i32, i32>::new();
        for task in tasks {
            *counts.entry(task).or_insert(0) += 1;
        }
        let mut total = 0;
        for (_, count) in counts {
            if count == 1 {
                return -1;
            }
            total += (count + 2) / 3;
        }
        total
    }
}
// @leetup=code

// @leetup=inject:after_code
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let tasks = vec![2, 2, 3, 3, 2, 4, 4, 4, 4, 4];
        let expected = 4;
        assert_eq!(Solution::minimum_rounds(tasks), expected);
    }

    #[test]
    fn example_2() {
        let tasks = vec![2, 3, 3];
        let expected = -1;
        assert_eq!(Solution::minimum_rounds(tasks), expected);
    }
}
// @leetup=inject:after_code
