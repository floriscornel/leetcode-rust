#![allow(dead_code, unused_variables)]
struct Solution {}

/** Problem Name - https://leetcode.com/problems/problem-name/ */

impl Solution {
    pub fn example_problem(i: i32) -> i32 {
        i - 1
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn example_problem() {
        assert_eq!(super::Solution::example_problem(5), 4);
    }
}
