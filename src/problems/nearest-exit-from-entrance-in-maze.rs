// @leetup=custom
// @leetup=info id=1926 lang=rust slug=nearest-exit-from-entrance-in-maze

/*
* You are given an `m x n` matrix `maze` (0-indexed) with empty cells
* (represented as `'.'`) and walls (represented as `'+'`). You are also given the
* `entrance` of the maze, where `entrance = [entrancerow, entrancecol]` denotes
* the row and column of the cell you are initially standing at.
*
* In one step, you can move one cell up, down, left, or right. You
* cannot step into a cell with a wall, and you cannot step outside the maze. Your
* goal is to find the nearest exit from the `entrance`. An exit is defined
* as an empty cell that is at the border of the `maze`. The `entrance`
* does not count as an exit.
*
* Return *the number of steps in the shortest path from the *`entrance`* to
* the nearest exit, or *`-1`* if no such path exists*.
*
*
* Example 1:
*
* []
* Input: maze = [["+","+",".","+"],[".",".",".","+"],["+","+","+","."]], entra
* nce = [1,2]
* Output: 1
* Explanation: There are 3 exits in this maze at [1,0], [0,2], and [2,3].
* Initially, you are at the entrance cell [1,2].
* - You can reach [1,0] by moving 2 steps left.
* - You can reach [0,2] by moving 1 step up.
* It is impossible to reach [2,3] from the entrance.
* Thus, the nearest exit is [0,2], which is 1 step away.
*
* Example 2:
*
* []
* Input: maze = [["+","+","+"],[".",".","."],["+","+","+"]], entrance = [1,0]
* Output: 2
* Explanation: There is 1 exit in this maze at [1,2].
* [1,0] does not count as an exit since it is the entrance cell.
* Initially, you are at the entrance cell [1,0].
* - You can reach [1,2] by moving 2 steps right.
* Thus, the nearest exit is [1,2], which is 2 steps away.
*
* Example 3:
*
* []
* Input: maze = [[".","+"]], entrance = [0,0]
* Output: -1
* Explanation: There are no exits in this maze.
*
*
* Constraints:
*
* * `maze.length == m`
* * `maze[i].length == n`
* * `1 <= m, n <= 100`
* * `maze[i][j]` is either `'.'` or `'+'`.
* * `entrance.length == 2`
* * `0 <= entrancerow < m`
* * `0 <= entrancecol < n`
* * `entrance` will always be an empty cell.
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
    pub fn nearest_exit(maze: Vec<Vec<char>>, entrance: Vec<i32>) -> i32 {
        type State = (i32, i32);
        let start_state: State = (entrance[0], entrance[1]);
        let mut queue = VecDeque::<(State, i32)>::new();
        let mut visited = HashSet::<State>::new();
        queue.push_back((start_state, 0));
        visited.insert(start_state);
        let (m, n) = (maze.len() as i32, maze[0].len() as i32);
        while !queue.is_empty() {
            let ((x, y), step_count) = queue.pop_front().unwrap();
            let possible_moves = [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)];
            for (new_x, new_y) in possible_moves {
                if new_x < 0
                    || new_x >= m
                    || new_y < 0
                    || new_y >= n
                    || maze[new_x as usize][new_y as usize] == '+'
                    || visited.contains(&(new_x, new_y))
                {
                    continue;
                }
                let new_sate = (new_x, new_y);
                if (new_x == 0 || new_x == m - 1 || new_y == 0 || new_y == n - 1)
                    && (new_x != start_state.0 || new_y != start_state.1)
                {
                    return step_count + 1;
                }
                visited.insert(new_sate);
                queue.push_back((new_sate, step_count + 1));
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
        let maze = vec![
            vec!['+', '+', '.', '+'],
            vec!['.', '.', '.', '+'],
            vec!['+', '+', '+', '.'],
        ];
        let entrance = vec![1, 2];
        assert_eq!(Solution::nearest_exit(maze, entrance), 1);
    }

    #[test]
    fn example_2() {
        let maze = vec![
            vec!['+', '+', '+'],
            vec!['.', '.', '.'],
            vec!['+', '+', '+'],
        ];
        let entrance = vec![1, 0];
        assert_eq!(Solution::nearest_exit(maze, entrance), 2);
    }

    #[test]
    fn example_3() {
        let maze = vec![vec!['.', '+']];
        let entrance = vec![0, 0];
        assert_eq!(Solution::nearest_exit(maze, entrance), -1);
    }
}
// @leetup=inject:after_code
