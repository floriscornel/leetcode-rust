// @leetup=custom
// @leetup=info id=37 lang=rust slug=sudoku-solver

/*
* Write a program to solve a Sudoku puzzle by filling the empty cells.
*
* A sudoku solution must satisfy all of the following rules:
*
* 1. Each of the digits `1-9` must occur exactly once in each row.
* 2. Each of the digits `1-9` must occur exactly once in each column.
* 3. Each of the digits `1-9` must occur exactly once in each of the 9 `3x3`
*    sub-boxes of the grid.
*
* The `'.'` character indicates empty cells.
*
*
* Example 1:
*
* Input: board = vec![['5','3','.','.','7','.','.','.','.'],['6','.','.','1','9','
* 5','.','.','.'],['.','9','8','.','.','.','.','6','.'],['8','.','.','.','6','.','
* .','.','3'],['4','.','.','8','.','3','.','.','1'],['7','.','.','.','2','.','.','
* .','6'],['.','6','.','.','.','.','2','8','.'],['.','.','.','4','1','9','.','.','
* 5'],['.','.','.','.','8','.','.','7','9']]
* Output: [['5','3','4','6','7','8','9','1','2'],['6','7','2','1','9','5','3',
* '4','8'],['1','9','8','3','4','2','5','6','7'],['8','5','9','7','6','1','4','2',
* '3'],['4','2','6','8','5','3','7','9','1'],['7','1','3','9','2','4','8','5','6']
* ,['9','6','1','5','3','7','2','8','4'],['2','8','7','4','1','9','6','3','5'],['3
* ','4','5','2','8','6','1','7','9']]
* Explanation: The input board is shown above and the only valid solution is s
* hown below:
*
*
* Constraints:
*
* * `board.length == 9`
* * `board[i].length == 9`
* * `board[i][j]` is a digit or `'.'`.
* * It is guaranteed that the input board has only one solution.
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
    // Return number of filled in squares
    pub fn completed(b: &[u8; 81]) -> usize {
        b.iter().filter(|x| **x > 0).count()
    }

    // For a board and position, return all valid options
    pub fn options(b: &[u8; 81], position: usize) -> Vec<u8> {
        let idx = |x, y| (x * 9 + y);
        let (row, column) = (position / 9, position % 9);
        let mut options = [true; 10];
        for i in 0..9 {
            options[b[idx(row, i)] as usize] = false;
            options[b[idx(i, column)] as usize] = false;
            options[b[idx((row / 3) * 3 + i % 3, (column / 3) * 3 + i / 3)] as usize] = false;
        }
        let mut res = vec![];
        for (value, is_option) in options.iter().enumerate() {
            if *is_option {
                res.push(value as u8);
            }
        }
        res
    }

    // Without back-tracking, fill in as much as possible rule-based.
    pub fn fill_in_rules(b: &mut [u8; 81]) {
        let (mut solved, mut last_solved) = (Solution::completed(b), usize::MAX);
        while solved != last_solved {
            for position in 0..81 {
                if b[position] == 0 {
                    let options = Solution::options(b, position);
                    if options.len() == 1 {
                        b[position] = options[0];
                    }
                }
            }
            last_solved = solved;
            solved = Solution::completed(b);
        }
    }

    // Recursively solve the problem
    pub fn do_best_guess(mut b: [u8; 81]) -> Option<[u8; 81]> {
        Solution::fill_in_rules(&mut b);
        if Solution::completed(&b) == 81 {
            return Some(b);
        }
        // Find the best spot to do a guess (least possible options)
        let (mut min_count, mut min_position) = (usize::MAX, 0);
        for position in 0..81 {
            if b[position] == 0 {
                let options = Solution::options(&b, position);
                if !options.is_empty() && options.len() < min_count {
                    min_count = options.len();
                    min_position = position;
                }
            }
        }
        if min_count != usize::MAX {
            for option in Solution::options(&b, min_position) {
                // Make a guess and see if it plays out
                b[min_position] = option;
                if let Some(solved_board) = Solution::do_best_guess(b) {
                    return Some(solved_board);
                }
            }
        }
        None
    }

    pub fn solve_sudoku(board: &mut [Vec<char>]) {
        let mut b: [u8; 81] = [0; 81];
        for (position, square) in b.iter_mut().enumerate() {
            if let Some(c) = board[position / 9][position % 9].to_digit(10) {
                *square = c as u8;
            }
        }
        for (position, square) in Solution::do_best_guess(b).unwrap().iter().enumerate() {
            board[position / 9][position % 9] = char::from_digit(*square as u32, 10).unwrap();
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
        let mut board = vec![
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

        let answer = vec![
            vec!['5', '3', '4', '6', '7', '8', '9', '1', '2'],
            vec!['6', '7', '2', '1', '9', '5', '3', '4', '8'],
            vec!['1', '9', '8', '3', '4', '2', '5', '6', '7'],
            vec!['8', '5', '9', '7', '6', '1', '4', '2', '3'],
            vec!['4', '2', '6', '8', '5', '3', '7', '9', '1'],
            vec!['7', '1', '3', '9', '2', '4', '8', '5', '6'],
            vec!['9', '6', '1', '5', '3', '7', '2', '8', '4'],
            vec!['2', '8', '7', '4', '1', '9', '6', '3', '5'],
            vec!['3', '4', '5', '2', '8', '6', '1', '7', '9'],
        ];

        Solution::solve_sudoku(&mut board);
        assert_eq!(board, answer);
    }

    #[test]
    fn example_2() {
        let mut board = vec![
            vec!['.', '.', '9', '7', '4', '8', '.', '.', '.'],
            vec!['7', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '2', '.', '1', '.', '9', '.', '.', '.'],
            vec!['.', '.', '7', '.', '.', '.', '2', '4', '.'],
            vec!['.', '6', '4', '.', '1', '.', '5', '9', '.'],
            vec!['.', '9', '8', '.', '.', '.', '3', '.', '.'],
            vec!['.', '.', '.', '8', '.', '3', '.', '2', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '6'],
            vec!['.', '.', '.', '2', '7', '5', '9', '.', '.'],
        ];

        let answer = vec![
            vec!['5', '1', '9', '7', '4', '8', '6', '3', '2'],
            vec!['7', '8', '3', '6', '5', '2', '4', '1', '9'],
            vec!['4', '2', '6', '1', '3', '9', '8', '7', '5'],
            vec!['3', '5', '7', '9', '8', '6', '2', '4', '1'],
            vec!['2', '6', '4', '3', '1', '7', '5', '9', '8'],
            vec!['1', '9', '8', '5', '2', '4', '3', '6', '7'],
            vec!['9', '7', '5', '8', '6', '3', '1', '2', '4'],
            vec!['8', '3', '2', '4', '9', '1', '7', '5', '6'],
            vec!['6', '4', '1', '2', '7', '5', '9', '8', '3'],
        ];

        Solution::solve_sudoku(&mut board);
        assert_eq!(board, answer);
    }
}
// @leetup=inject:after_code
