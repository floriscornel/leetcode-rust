#![allow(dead_code, unused_variables)]

struct Solution {}

/** Problem Name - https://leetcode.com/problems/problem-name/ */

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

#[cfg(test)]
mod tests {
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
        assert_eq!(super::Solution::subsets(input), output);
    }

    #[test]
    fn example2() {
        let input = vec![0];
        let output = vec![vec![], vec![0]];
        assert_eq!(super::Solution::subsets(input), output);
    }
}
