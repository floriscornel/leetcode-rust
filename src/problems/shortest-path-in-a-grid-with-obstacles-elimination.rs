// @leetup=custom
// @leetup=info id=1293 lang=rust slug=shortest-path-in-a-grid-with-obstacles-elimination

/*
* You are given an `m x n` integer matrix `grid` where each cell is either `0`
* (empty) or `1` (obstacle). You can move up, down, left, or right from and to an
* empty cell in one step.
*
* Return *the minimum number of steps to walk from the upper left corner *`(0,
* 0)`* to the lower right corner *`(m - 1, n - 1)`* given that you can eliminate
* at most *`k`* obstacles*. If it is not possible to find such walk return
* `-1`.
*
*
* Example 1:
*
* []
* Input: grid = [[0,0,0],[1,1,0],[0,0,0],[0,1,1],[0,0,0]], k = 1
* Output: 6
* Explanation:
* The shortest path without eliminating any obstacle is 10.
* The shortest path with one obstacle elimination at position (3,2) is 6. Such pat
* h is (0,0) -> (0,1) -> (0,2) -> (1,2) -> (2,2) -> (3,2) -> (4,2).
*
* Example 2:
*
* []
* Input: grid = [[0,1,1],[1,1,1],[1,0,0]], k = 1
* Output: -1
* Explanation: We need to eliminate at least two obstacles to find such a walk
* .
*
*
* Constraints:
*
* * `m == grid.length`
* * `n == grid[i].length`
* * `1 <= m, n <= 40`
* * `1 <= k <= m * n`
* * `grid[i][j]` is either `0` or `1`.
* * `grid[0][0] == grid[m - 1][n - 1] == 0`
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
type State = (i32, i32, i32);
impl Solution {
    pub fn shortest_path(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let (m, n) = (grid.len() as i32, grid[0].len() as i32);
        let moves = [(0, -1), (-1, 0), (0, 1), (1, 0)];

        let mut states: VecDeque<(i32, State)> = VecDeque::new();
        let mut visited: HashSet<State> = HashSet::new();
        states.push_front((0, (n - 1, m - 1, k)));
        visited.insert((n - 1, m - 1, k));

        while !states.is_empty() {
            let (distance, state) = states.pop_front().unwrap();
            if state.0 == 0 && state.1 == 0 {
                return distance;
            }
            for (dy, dx) in moves {
                let (x, y) = (state.0 + dx, state.1 + dy);
                if x >= 0 && x < n && y >= 0 && y < m {
                    let k = state.2 - grid[y as usize][x as usize];
                    if k >= 0 && !visited.contains(&(x, y, k)) {
                        states.push_back((distance + 1, (x, y, k)));
                        visited.insert((x, y, k));
                    }
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
        assert_eq!(
            Solution::shortest_path(
                vec![
                    vec![0, 0, 0],
                    vec![1, 1, 0],
                    vec![0, 0, 0],
                    vec![0, 1, 1],
                    vec![0, 0, 0],
                ],
                1,
            ),
            6
        )
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::shortest_path(vec![vec![0, 1, 1], vec![1, 1, 1], vec![1, 0, 0]], 1,),
            -1
        )
    }

    #[test]
    fn example_3() {
        assert_eq!(
            Solution::shortest_path(
                vec![
                    vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    vec![0, 1, 1, 1, 1, 1, 1, 1, 1, 0],
                    vec![0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
                    vec![0, 1, 0, 1, 1, 1, 1, 1, 1, 1],
                    vec![0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
                    vec![0, 1, 1, 1, 1, 1, 1, 1, 1, 0],
                    vec![0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
                    vec![0, 1, 0, 1, 1, 1, 1, 1, 1, 1],
                    vec![0, 1, 0, 1, 1, 1, 1, 0, 0, 0],
                    vec![0, 1, 0, 0, 0, 0, 0, 0, 1, 0],
                    vec![0, 1, 1, 1, 1, 1, 1, 0, 1, 0],
                    vec![0, 0, 0, 0, 0, 0, 0, 0, 1, 0]
                ],
                1
            ),
            20
        )
    }

    #[test]
    fn example_4() {
        assert_eq!(
            Solution::shortest_path(
                vec![
                    vec![
                        0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1,
                        1, 1, 1, 1, 0, 0, 0, 1, 1
                    ],
                    vec![
                        1, 1, 1, 1, 1, 0, 0, 1, 0, 0, 1, 1, 0, 1, 0, 1, 1, 0, 1, 0, 0, 1, 0, 0, 1,
                        0, 1, 1, 1, 0, 1, 0, 0, 0
                    ],
                    vec![
                        1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 1, 1, 0, 0, 1, 0, 1, 1, 1, 1, 0, 0,
                        0, 0, 0, 0, 1, 1, 1, 0, 0
                    ],
                    vec![
                        0, 0, 1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1,
                        0, 0, 1, 0, 0, 1, 1, 0, 0
                    ],
                    vec![
                        1, 1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1,
                        0, 0, 0, 1, 0, 0, 0, 1, 0
                    ],
                    vec![
                        0, 1, 0, 1, 0, 0, 0, 1, 0, 1, 1, 1, 0, 0, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1,
                        1, 1, 1, 0, 0, 0, 0, 1, 1
                    ],
                    vec![
                        0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 1, 0, 1, 1, 0, 1, 1, 1, 0, 0, 1, 1,
                        1, 1, 1, 0, 1, 0, 0, 1, 0
                    ],
                    vec![
                        1, 0, 0, 1, 0, 0, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1, 0, 1, 1, 0, 0,
                        1, 0, 0, 1, 0, 0, 1, 0, 1
                    ],
                    vec![
                        1, 1, 1, 0, 0, 1, 0, 0, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0,
                        1, 0, 0, 0, 1, 0, 0, 1, 0
                    ]
                ],
                283
            ),
            41
        )
    }
}
// @leetup=inject:after_code
