// @leetup=custom
// @leetup=info id=36 lang=rust slug=valid-sudoku

/*
* Determine if a `9 x 9` Sudoku board is valid. Only the filled cells need to be
* validated according to the following rules:
*
* 1. Each row must contain the digits `1-9` without repetition.
* 2. Each column must contain the digits `1-9` without repetition.
* 3. Each of the nine `3 x 3` sub-boxes of the grid must contain the digits `1-9`
*    without repetition.
*
* Note:
*
* * A Sudoku board (partially filled) could be valid but is not necessarily
*   solvable.
* * Only the filled cells need to be validated according to the mentioned rules.
*
*
* Example 1:
*
* Input: board =
* [["5","3",".",".","7",".",".",".","."]
* ,["6",".",".","1","9","5",".",".","."]
* ,[".","9","8",".",".",".",".","6","."]
* ,["8",".",".",".","6",".",".",".","3"]
* ,["4",".",".","8",".","3",".",".","1"]
* ,["7",".",".",".","2",".",".",".","6"]
* ,[".","6",".",".",".",".","2","8","."]
* ,[".",".",".","4","1","9",".",".","5"]
* ,[".",".",".",".","8",".",".","7","9"]]
* Output: true
*
* Example 2:
*
* Input: board =
* [["8","3",".",".","7",".",".",".","."]
* ,["6",".",".","1","9","5",".",".","."]
* ,[".","9","8",".",".",".",".","6","."]
* ,["8",".",".",".","6",".",".",".","3"]
* ,["4",".",".","8",".","3",".",".","1"]
* ,["7",".",".",".","2",".",".",".","6"]
* ,[".","6",".",".",".",".","2","8","."]
* ,[".",".",".","4","1","9",".",".","5"]
* ,[".",".",".",".","8",".",".","7","9"]]
* Output: false
* Explanation: Same as Example 1, except with the 5 in the top left corner
*  being modified to 8. Since there are two 8's in the top left 3x3 sub-box, i
* t is invalid.
*
*
* Constraints:
*
* * `board.length == 9`
* * `board[i].length == 9`
* * `board[i][j]` is a digit `1-9` or `'.'`.
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
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut b: [u8; 81] = [0; 81];
        for (position, square) in b.iter_mut().enumerate() {
            if let Some(c) = board[position / 9][position % 9].to_digit(10) {
                *square = c as u8;
            }
        }

        for i in 0..9 {
            let mut found_row = [false; 10];
            let mut found_column = [false; 10];
            let mut found_square = [false; 10];
            let (sq_row, sq_col) = ((i % 3) * 3, (i / 3) * 3);

            for j in 0..9 {
                let row_value = b[i * 9 + j] as usize;
                let column_value = b[j * 9 + i] as usize;
                let square_value = b[(sq_row + j % 3) * 9 + sq_col + j / 3] as usize;

                if row_value > 0 && found_row[row_value] {
                    return false;
                }
                if column_value > 0 && found_column[column_value] {
                    return false;
                }
                if square_value > 0 && found_square[square_value] {
                    return false;
                }

                found_row[row_value] = true;
                found_square[square_value] = true;
                found_column[column_value] = true;
            }
        }
        true
    }
}
// @leetup=code

// @leetup=inject:after_code
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let board = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        assert!(Solution::is_valid_sudoku(board));
    }

    #[test]
    fn example_2() {
        let board = vec![
            vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        assert!(!Solution::is_valid_sudoku(board));
    }
}
// @leetup=inject:after_code
