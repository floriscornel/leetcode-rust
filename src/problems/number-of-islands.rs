// @leetup=custom
// @leetup=info id=200 lang=rust slug=number-of-islands

/*
* Given an `m x n` 2D binary grid `grid` which represents a map of `'1'`s (land)
* and `'0'`s (water), return *the number of islands*.
*
* An island is surrounded by water and is formed by connecting adjacent lands
* horizontally or vertically. You may assume all four edges of the grid are all
* surrounded by water.
*
*
* Example 1:
*
* Input: grid = [
*   ["1","1","1","1","0"],
*   ["1","1","0","1","0"],
*   ["1","1","0","0","0"],
*   ["0","0","0","0","0"]
* ]
* Output: 1
*
* Example 2:
*
* Input: grid = [
*   ["1","1","0","0","0"],
*   ["1","1","0","0","0"],
*   ["0","0","1","0","0"],
*   ["0","0","0","1","1"]
* ]
* Output: 3
*
*
* Constraints:
*
* * `m == grid.length`
* * `n == grid[i].length`
* * `1 <= m, n <= 300`
* * `grid[i][j]` is `'0'` or `'1'`.
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
    fn explore(grid: &mut [Vec<char>], x: usize, y: usize) {
        if grid[y][x] == '1' {
            grid[y][x] = 'e';
            if x > 0 {
                Solution::explore(grid, x - 1, y);
            }
            if y > 0 {
                Solution::explore(grid, x, y - 1);
            }
            if x < grid[0].len() - 1 {
                Solution::explore(grid, x + 1, y);
            }
            if y < grid.len() - 1 {
                Solution::explore(grid, x, y + 1);
            }
        }
    }
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        let mut island_count = 0;
        for y in 0..grid.len() {
            for x in 0..grid[0].len() {
                if grid[y][x] == '1' {
                    Solution::explore(&mut grid, x, y);
                    island_count += 1;
                }
            }
        }
        island_count
    }
}
// @leetup=code

// @leetup=inject:after_code
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let grid = vec![
            vec!['1', '1', '1', '1', '0'],
            vec!['1', '1', '0', '1', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '0', '0', '0'],
        ];
        assert_eq!(Solution::num_islands(grid), 1)
    }

    #[test]
    fn example_2() {
        let grid = vec![
            vec!['1', '1', '0', '0', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '1', '0', '0'],
            vec!['0', '0', '0', '1', '1'],
        ];
        assert_eq!(Solution::num_islands(grid), 3)
    }
}
// @leetup=inject:after_code
