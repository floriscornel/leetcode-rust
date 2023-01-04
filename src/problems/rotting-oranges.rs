// @leetup=custom
// @leetup=info id=994 lang=rust slug=rotting-oranges

/*
* You are given an `m x n` `grid` where each cell can have one of three values:
*
* * `0` representing an empty cell,
* * `1` representing a fresh orange, or
* * `2` representing a rotten orange.
*
* Every minute, any fresh orange that is 4-directionally adjacent to a rotten
* orange becomes rotten.
*
* Return *the minimum number of minutes that must elapse until no cell has a fresh
* orange*. If *this is impossible, return* `-1`.
*
*
* Example 1:
*
* []
* Input: grid = [[2,1,1],[1,1,0],[0,1,1]]
* Output: 4
*
* Example 2:
*
* Input: grid = [[2,1,1],[0,1,1],[1,0,1]]
* Output: -1
* Explanation: The orange in the bottom left corner (row 2, column 0) is never
*  rotten, because rotting only happens 4-directionally.
*
* Example 3:
*
* Input: grid = [[0,2]]
* Output: 0
* Explanation: Since there are already no fresh oranges at minute 0, the answe
* r is just 0.
*
*
* Constraints:
*
* * `m == grid.length`
* * `n == grid[i].length`
* * `1 <= m, n <= 10`
* * `grid[i][j]` is `0`, `1`, or `2`.
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
    pub fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut queue = Vec::<(usize, usize)>::new();
        let mut fresh_count = 0;
        for (i, col) in grid.iter().enumerate() {
            for (j, elem) in col.iter().enumerate() {
                if *elem == 2 {
                    queue.push((i, j));
                } else if *elem == 1 {
                    fresh_count += 1;
                }
            }
        }
        let mut minutes = 0;
        while !queue.is_empty() {
            let mut new_queue = Vec::new();
            for (i, j) in queue {
                if i > 0 && grid[i - 1][j] == 1 {
                    grid[i - 1][j] = 2;
                    fresh_count -= 1;
                    new_queue.push((i - 1, j));
                }
                if i + 1 < grid.len() && grid[i + 1][j] == 1 {
                    grid[i + 1][j] = 2;
                    fresh_count -= 1;
                    new_queue.push((i + 1, j));
                }
                if j > 0 && grid[i][j - 1] == 1 {
                    grid[i][j - 1] = 2;
                    fresh_count -= 1;
                    new_queue.push((i, j - 1));
                }
                if j + 1 < grid[i].len() && grid[i][j + 1] == 1 {
                    grid[i][j + 1] = 2;
                    fresh_count -= 1;
                    new_queue.push((i, j + 1));
                }
            }
            queue = new_queue;
            minutes += 1;
        }
        if fresh_count > 0 {
            -1
        } else {
            minutes - 1
        }
    }
}
// @leetup=code

// @leetup=inject:after_code
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let grid = vec![vec![2, 1, 1], vec![1, 1, 0], vec![0, 1, 1]];
        assert_eq!(Solution::oranges_rotting(grid), 4);
    }

    #[test]
    fn example_2() {
        let grid = vec![vec![2, 1, 1], vec![0, 1, 1], vec![1, 0, 1]];
        assert_eq!(Solution::oranges_rotting(grid), -1);
    }
}
// @leetup=inject:after_code
