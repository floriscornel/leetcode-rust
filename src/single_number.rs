#![allow(dead_code, unused_variables)]

struct Solution {}

/** 136. Single Number - https://leetcode.com/problems/single-number/submissions/ */
use std::collections::HashSet;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut set: HashSet<i32> = HashSet::with_capacity(nums.len());
        for num in nums {
            if set.contains(&num) {
                set.remove(&num);
            } else {
                set.insert(num);
            }
        }
        *set.iter().next().unwrap()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn example_problem() {
        assert_eq!(super::Solution::single_number(vec![2, 2, 1]), 1);
        assert_eq!(super::Solution::single_number(vec![4, 1, 2, 1, 2]), 4);
        assert_eq!(super::Solution::single_number(vec![1]), 1);
    }
}
