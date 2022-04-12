// @leetup=custom
// @leetup=info id=289 lang=rust slug=game-of-life

/*
* According to [Wikipedia's article][1]: "The Game of Life, also known simply as
* Life, is a cellular automaton devised by the British mathematician John Horton
* Conway in 1970."
*
* The board is made up of an `m x n` grid of cells, where each cell has an initial
* state: live (represented by a `1`) or dead (represented by a `0`). Each cell
* interacts with its [eight neighbors][2] (horizontal, vertical, diagonal) using
* the following four rules (taken from the above Wikipedia article):
*
* 1. Any live cell with fewer than two live neighbors dies as if caused by
*    under-population.
* 2. Any live cell with two or three live neighbors lives on to the next
*    generation.
* 3. Any live cell with more than three live neighbors dies, as if by
*    over-population.
* 4. Any dead cell with exactly three live neighbors becomes a live cell, as if by
*    reproduction.
*
* The next state is created by applying the above rules simultaneously to every
* cell in the current state, where births and deaths occur simultaneously. Given
* the current state of the `m x n` grid `board`, return *the next state*.
*
*
* Example 1:
*
* []
* Input: board = [[0,1,0],[0,0,1],[1,1,1],[0,0,0]]
* Output: [[0,0,0],[1,0,1],[0,1,1],[0,1,0]]
*
* Example 2:
*
* []
* Input: board = [[1,1],[1,0]]
* Output: [[1,1],[1,1]]
*
*
* Constraints:
*
* * `m == board.length`
* * `n == board[i].length`
* * `1 <= m, n <= 25`
* * `board[i][j]` is `0` or `1`.
*
*
* Follow up:
*
* * Could you solve it in-place? Remember that the board needs to be updated
*   simultaneously: You cannot update some cells first and then use their updated
*   values to update other cells.
* * In this question, we represent the board using a 2D array. In principle, the
*   board is infinite, which would cause problems when the active area encroaches
*   upon the border of the array (i.e., live cells reach the border). How would
*   you address these problems?
*
* [1] https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life
* [2] https://en.wikipedia.org/wiki/Moore_neighborhood
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
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        let (m, n) = (board.len(), board[0].len());
        let mut changes: Vec<(usize, usize)> = Vec::new();

        for i in 0..m {
            for j in 0..n {
                let neighbor_count = Solution::count_neighbors(board, i, j, m, n);

                let new_state = if neighbor_count == 3 {
                    1
                } else if neighbor_count == 2 {
                    board[i][j]
                } else {
                    0
                };

                if new_state != board[i][j] {
                    changes.push((i, j));
                }
            }
        }

        for (i, j) in changes {
            board[i][j] = if board[i][j] == 1 { 0 } else { 1 };
        }
    }

    fn count_neighbors(board: &[Vec<i32>], i: usize, j: usize, m: usize, n: usize) -> i32 {
        let mut neighbors = 0;
        if i > 0 {
            if board[i - 1][j] == 1 {
                neighbors += 1;
            }
            if j > 0 && board[i - 1][j - 1] == 1 {
                neighbors += 1;
            }
            if j < n - 1 && board[i - 1][j + 1] == 1 {
                neighbors += 1;
            }
        }
        if i < m - 1 {
            if board[i + 1][j] == 1 {
                neighbors += 1;
            }
            if j > 0 && board[i + 1][j - 1] == 1 {
                neighbors += 1;
            }
            if j < n - 1 && board[i + 1][j + 1] == 1 {
                neighbors += 1;
            }
        }
        if j > 0 && board[i][j - 1] == 1 {
            neighbors += 1;
        }
        if j < n - 1 && board[i][j + 1] == 1 {
            neighbors += 1;
        }
        neighbors
    }
}
// @leetup=code

// @leetup=inject:after_code
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let mut input = vec![vec![0, 1, 0], vec![0, 0, 1], vec![1, 1, 1], vec![0, 0, 0]];
        let output = vec![vec![0, 0, 0], vec![1, 0, 1], vec![0, 1, 1], vec![0, 1, 0]];
        Solution::game_of_life(&mut input);
        assert_eq!(input, output);
    }

    #[test]
    fn example_2() {
        let mut input = vec![vec![1, 1], vec![1, 0]];
        let output = vec![vec![1, 1], vec![1, 1]];
        Solution::game_of_life(&mut input);
        assert_eq!(input, output);
    }
}
// @leetup=inject:after_code
