// @leetup=custom
// @leetup=info id=79 lang=rust slug=word-search

/*
* Given an `m x n` grid of characters `board` and a string `word`, return `true`
* *if* `word` *exists in the grid*.
*
* The word can be constructed from letters of sequentially adjacent cells, where
* adjacent cells are horizontally or vertically neighboring. The same letter cell
* may not be used more than once.
*
*
* Example 1:
*
* []
* Input: board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word
*  = "ABCCED"
* Output: true
*
* Example 2:
*
* []
* Input: board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word
*  = "SEE"
* Output: true
*
* Example 3:
*
* []
* Input: board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word
*  = "ABCB"
* Output: false
*
*
* Constraints:
*
* * `m == board.length`
* * `n = board[i].length`
* * `1 <= m, n <= 6`
* * `1 <= word.length <= 15`
* * `board` and `word` consists of only lowercase and uppercase English letters.
*
*
* Follow up: Could you use search pruning to make your solution faster with a
* larger `board`?
*
*/
// @leetup=custom
// @leetup=inject:before_code_ex
// before_code_exclude
#![allow(dead_code, unused_variables)]

struct Solution {}
// @leetup=inject:before_code_ex

// @leetup=code
type TravelSequence = Vec<(usize, usize)>;

impl Solution {
    fn get_options(sequence: &TravelSequence, m: usize, n: usize) -> TravelSequence {
        let &(i, j) = sequence.last().unwrap();
        let mut options: TravelSequence = Vec::new();
        if i > 0 && !sequence.contains(&(i - 1, j)) {
            options.push((i - 1, j));
        }
        if j > 0 && !sequence.contains(&(i, j - 1)) {
            options.push((i, j - 1));
        }
        if i + 1 < m && !sequence.contains(&(i + 1, j)) {
            options.push((i + 1, j));
        }
        if j + 1 < n && !sequence.contains(&(i, j + 1)) {
            options.push((i, j + 1));
        }
        options
    }

    pub fn follow_sequence(
        board: &[Vec<char>],
        answer: &str,
        sequence: &mut TravelSequence,
        word: &mut String,
    ) -> bool {
        if sequence.len() > answer.len() {
            return false;
        }
        if sequence.len() == answer.len() {
            return true;
        }
        let expected_char = answer.as_bytes()[sequence.len()];
        let options = Self::get_options(sequence, board.len(), board[0].len());
        for (x, y) in options {
            let found_char = board[x][y] as u8;
            if found_char == expected_char {
                word.push(found_char as char);
                sequence.push((x, y));
                if Self::follow_sequence(board, answer, sequence, word) {
                    return true;
                }
                word.pop();
                sequence.pop();
            }
        }
        false
    }

    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let first = word.as_bytes()[0] as char;
        for (i, row) in board.iter().enumerate() {
            for (j, char) in row.iter().enumerate() {
                if *char == first
                    && Self::follow_sequence(
                        &board,
                        &word,
                        &mut vec![(i, j)],
                        &mut char.to_string(),
                    )
                {
                    return true;
                }
            }
        }
        false
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
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];
        let word = "ABCCED".to_owned();
        assert!(Solution::exist(board, word))
    }

    #[test]
    fn example_2() {
        let board = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];
        let word = "SEE".to_owned();
        assert!(Solution::exist(board, word))
    }

    #[test]
    fn example_3() {
        let board = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];
        let word = "ABCB".to_owned();
        assert!(!Solution::exist(board, word))
    }

    #[test]
    fn example_4() {
        let board = vec![
            vec!['A', 'A', 'A', 'A'],
            vec!['A', 'A', 'A', 'A'],
            vec!['A', 'A', 'A', 'A'],
            vec!['A', 'A', 'A', 'A'],
            vec!['A', 'A', 'A', 'A'],
        ];
        let word = "BAAAAAAAAAAAAAA".to_owned();
        assert!(!Solution::exist(board, word))
    }

    #[test]
    fn example_5() {
        let board = vec![vec!['A']];
        let word = "B".to_owned();
        assert!(!Solution::exist(board, word))
    }

    #[test]
    fn example_6() {
        let board = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'E', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];
        let word = "ABCESEEEFS".to_owned();
        assert!(Solution::exist(board, word))
    }
}
// @leetup=inject:after_code
