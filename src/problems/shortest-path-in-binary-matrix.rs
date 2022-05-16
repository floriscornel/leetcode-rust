// @leetup=custom
// @leetup=info id=1091 lang=rust slug=shortest-path-in-binary-matrix

/*
* Given an `n x n` binary matrix `grid`, return *the length of the shortest
* clear path in the matrix*. If there is no clear path, return `-1`.
*
* A clear path in a binary matrix is a path from the top-left cell (i.e.,
* `(0, 0)`) to the bottom-right cell (i.e., `(n - 1, n - 1)`) such that:
*
* * All the visited cells of the path are `0`.
* * All the adjacent cells of the path are 8-directionally connected (i.e.,
*   they are different and they share an edge or a corner).
*
* The length of a clear path is the number of visited cells of this path.
*
*
* Example 1:
*
* []
* Input: grid = [[0,1],[1,0]]
* Output: 2
*
* Example 2:
*
* []
* Input: grid = [[0,0,0],[1,1,0],[1,1,0]]
* Output: 4
*
* Example 3:
*
* Input: grid = [[1,0,0],[1,1,0],[1,1,0]]
* Output: -1
*
*
* Constraints:
*
* * `n == grid.length`
* * `n == grid[i].length`
* * `1 <= n <= 100`
* * `grid[i][j] is 0 or 1`
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

type Point = (i32, i32);

impl Solution {
    pub fn shortest_path_binary_matrix(grid: Vec<Vec<i32>>) -> i32 {
        let value = |p: Point| grid[p.0 as usize][p.1 as usize];

        let (initial_state, finished_state) =
            ((0, 0), (grid.len() as i32 - 1, grid[0].len() as i32 - 1));

        if value(initial_state) == 1 || value(finished_state) == 1 {
            return -1;
        }

        if initial_state == finished_state {
            return 1;
        }

        let mut states: VecDeque<(Point, i32)> = VecDeque::new();
        states.push_back((initial_state, 1));
        let mut visited: HashSet<Point> = HashSet::new();
        visited.insert(initial_state);

        while !states.is_empty() {
            let (state, distance) = states.pop_front().unwrap();
            for step in [
                (0, 1),
                (1, 1),
                (1, 0),
                (1, -1),
                (0, -1),
                (-1, -1),
                (-1, 0),
                (-1, 1),
            ] {
                let new_state = (
                    i32::min(i32::max(0, state.0 + step.0), finished_state.0),
                    i32::min(i32::max(0, state.1 + step.1), finished_state.1),
                );
                if new_state == finished_state {
                    return distance + 1;
                }
                if value(new_state) == 0 && !visited.contains(&new_state) {
                    visited.insert(new_state);
                    states.push_back((new_state, distance + 1));
                }
            }
        }
        -1
    }
}
// @leetup=code

// @leetup=inject:after_code
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let grid = vec![vec![0, 1], vec![1, 0]];
        assert_eq!(Solution::shortest_path_binary_matrix(grid), 2);
    }

    #[test]
    fn example_2() {
        let grid = vec![vec![0, 0, 0], vec![1, 1, 0], vec![1, 1, 0]];
        assert_eq!(Solution::shortest_path_binary_matrix(grid), 4);
    }

    #[test]
    fn example_3() {
        let grid = vec![vec![1, 0, 0], vec![1, 1, 0], vec![1, 1, 0]];
        assert_eq!(Solution::shortest_path_binary_matrix(grid), -1);
    }

    #[test]
    fn example_4() {
        let grid = vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]];
        assert_eq!(Solution::shortest_path_binary_matrix(grid), 3);
    }

    #[test]
    fn example_5() {
        let grid = vec![vec![0]];
        assert_eq!(Solution::shortest_path_binary_matrix(grid), 1);
    }
}
// @leetup=inject:after_code
