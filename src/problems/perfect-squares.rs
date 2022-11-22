// @leetup=custom
// @leetup=info id=279 lang=rust slug=perfect-squares

/*
* Given an integer `n`, return *the least number of perfect square numbers that
* sum to* `n`.
*
* A perfect square is an integer that is the square of an integer; in other
* words, it is the product of some integer with itself. For example, `1`, `4`,
* `9`, and `16` are perfect squares while `3` and `11` are not.
*
*
* Example 1:
*
* Input: n = 12
* Output: 3
* Explanation: 12 = 4 + 4 + 4.
*
* Example 2:
*
* Input: n = 13
* Output: 2
* Explanation: 13 = 4 + 9.
*
*
* Constraints:
*
* * `1 <= n <= 104`
*
*/
// @leetup=custom
// @leetup=inject:before_code_ex
// before_code_exclude
#![allow(dead_code, unused_variables)]

struct Solution {}
// @leetup=inject:before_code_ex

// @leetup=code
use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let mut square = 1;
        let mut squares = vec![];
        while square * square < n {
            squares.push(square * square);
            square += 1;
        }

        type State = (i32, i32);
        let initial_state = (0, 0);
        let mut queue = VecDeque::<State>::new();
        let mut visited = HashSet::<State>::new();
        queue.push_back(initial_state);
        visited.insert(initial_state);
        while !queue.is_empty() {
            let (num, count) = queue.pop_front().unwrap();
            for square in squares.iter().rev() {
                let new_state = (num + square, count + 1);
                if new_state.0 == n {
                    return count + 1;
                }
                if new_state.0 > n || visited.contains(&new_state) {
                    continue;
                }
                queue.push_back(new_state);
            }
        }
        1
    }
}
// @leetup=code

// @leetup=inject:after_code
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        assert_eq!(Solution::num_squares(12), 3);
        assert_eq!(Solution::num_squares(13), 2);
    }
}
// @leetup=inject:after_code
