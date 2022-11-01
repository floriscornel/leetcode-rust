// @leetup=custom
// @leetup=info id=1706 lang=rust slug=where-will-the-ball-fall

/*
* You have a 2-D `grid` of size `m x n` representing a box, and you have `n`
* balls. The box is open on the top and bottom sides.
*
* Each cell in the box has a diagonal board spanning two corners of the cell that
* can redirect a ball to the right or to the left.
*
* * A board that redirects the ball to the right spans the top-left corner to the
*   bottom-right corner and is represented in the grid as `1`.
* * A board that redirects the ball to the left spans the top-right corner to the
*   bottom-left corner and is represented in the grid as `-1`.
*
* We drop one ball at the top of each column of the box. Each ball can get stuck
* in the box or fall out of the bottom. A ball gets stuck if it hits a "V" shaped
* pattern between two boards or if a board redirects the ball into either wall of
* the box.
*
* Return *an array *`answer`* of size *`n`* where *`answer[i]`* is the column that
* the ball falls out of at the bottom after dropping the ball from the *`ith`*
* column at the top, or `-1`* if the ball gets stuck in the box*.*
*
*
* Example 1:
*
* []
*
* Input: grid = [[1,1,1,-1,-1],[1,1,1,-1,-1],[-1,-1,-1,1,1],[1,1,1,1,-1],[-1,-
* 1,-1,-1,-1]]
* Output: [1,-1,-1,-1,-1]
* Explanation: This example is shown in the photo.
* Ball b0 is dropped at column 0 and falls out of the box at column 1.
* Ball b1 is dropped at column 1 and will get stuck in the box between column 2 an
* d 3 and row 1.
* Ball b2 is dropped at column 2 and will get stuck on the box between column 2 an
* d 3 and row 0.
* Ball b3 is dropped at column 3 and will get stuck on the box between column 2 an
* d 3 and row 0.
* Ball b4 is dropped at column 4 and will get stuck on the box between column 2 an
* d 3 and row 1.
*
* Example 2:
*
* Input: grid = [[-1]]
* Output: [-1]
* Explanation: The ball gets stuck against the left wall.
*
* Example 3:
*
* Input: grid = [[1,1,1,1,1,1],[-1,-1,-1,-1,-1,-1],[1,1,1,1,1,1],[-1,-1,-1,-1,
* -1,-1]]
* Output: [0,1,2,3,4,-1]
*
*
* Constraints:
*
* * `m == grid.length`
* * `n == grid[i].length`
* * `1 <= m, n <= 100`
* * `grid[i][j]` is `1` or `-1`.
*
*/
// @leetup=custom
// @leetup=inject:before_code_ex
// before_code_exclude
#![allow(dead_code, unused_variables)]
struct Solution {}
// @leetup=inject:before_code_ex

// @leetup=code

use Direction::{Left, Right, Wall};
enum Direction {
    Left,
    Right,
    Wall,
}

impl Solution {
    pub fn find_ending(grid: &[Vec<i32>], row: usize, col: i32) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        if row == m {
            return col;
        }
        let direction = |i| {
            if i < 0 || i >= n as i32 {
                Wall
            } else if grid[row][i as usize] == 1 {
                Right
            } else {
                Left
            }
        };
        let (left, middle, right) = (direction(col - 1), direction(col), direction(col + 1));
        match (left, middle, right) {
            (Left, Left, _) => Self::find_ending(grid, row + 1, col - 1),
            (_, Right, Right) => Self::find_ending(grid, row + 1, col + 1),
            _ => -1,
        }
    }

    pub fn find_ball(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let (m, n) = (grid.len(), grid[0].len());
        (0..n)
            .into_iter()
            .map(|start| Self::find_ending(&grid, 0, start as i32))
            .collect()
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
            Solution::find_ball(vec![
                vec![1, 1, 1, -1, -1],
                vec![1, 1, 1, -1, -1],
                vec![-1, -1, -1, 1, 1],
                vec![1, 1, 1, 1, -1],
                vec![-1, -1, -1, -1, -1]
            ]),
            vec![1, -1, -1, -1, -1]
        )
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::find_ball(vec![vec![-1]]), vec![-1])
    }

    #[test]
    fn example_3() {
        assert_eq!(
            Solution::find_ball(vec![
                vec![1, 1, 1, 1, 1, 1],
                vec![-1, -1, -1, -1, -1, -1],
                vec![1, 1, 1, 1, 1, 1],
                vec![-1, -1, -1, -1, -1, -1]
            ]),
            vec![0, 1, 2, 3, 4, -1]
        )
    }

    #[test]
    fn example_4() {
        assert_eq!(
            Solution::find_ball(vec![vec![
                -1, 1, -1, -1, -1, -1, -1, -1, 1, -1, -1, -1, -1, 1, 1, -1, -1, -1, 1, 1, 1, -1,
                -1, 1, 1, -1, -1, 1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 1, -1, 1, -1, -1, -1, -1,
                -1, -1, -1, 1, -1, -1, 1, -1, 1, -1, -1, 1, 1, -1, 1, -1, -1, -1, -1, 1, 1, 1, 1,
                1, 1, -1, 1, 1, 1, -1, 1, 1, 1, -1, -1, -1, 1, -1, 1, -1, -1, 1, 1, -1, -1, 1, -1,
                1, -1, 1, 1, 1, -1, -1, -1, -1
            ]]),
            vec![
                -1, -1, -1, 2, 3, 4, 5, 6, -1, -1, 9, 10, 11, 14, -1, -1, 15, 16, 19, 20, -1, -1,
                21, 24, -1, -1, 25, -1, -1, 28, 29, 30, 31, 32, 33, 34, 35, -1, -1, -1, -1, 40, 41,
                42, 43, 44, 45, -1, -1, 48, -1, -1, -1, -1, 53, 56, -1, -1, -1, -1, 59, 60, 61, 64,
                65, 66, 67, 68, -1, -1, 71, 72, -1, -1, 75, 76, -1, -1, 77, 78, -1, -1, -1, -1, 83,
                86, -1, -1, 87, -1, -1, -1, -1, 94, 95, -1, -1, 96, 97, 98
            ]
        )
    }
}
// @leetup=inject:after_code
