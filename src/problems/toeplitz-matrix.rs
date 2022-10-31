// @leetup=custom
// @leetup=info id=766 lang=rust slug=toeplitz-matrix

/*
* Given an `m x n` `matrix`, return *`true` if the matrix is Toeplitz. Otherwise,
* return `false`.*
*
* A matrix is Toeplitz if every diagonal from top-left to bottom-right has the
* same elements.
*
*
* Example 1:
*
* []
* Input: matrix = [[1,2,3,4],[5,1,2,3],[9,5,1,2]]
* Output: true
* Explanation:
* In the above grid, the diagonals are:
* "[9]", "[5, 5]", "[1, 1, 1]", "[2, 2, 2]", "[3, 3]", "[4]".
* In each diagonal all elements are the same, so the answer is True.
*
* Example 2:
*
* []
* Input: matrix = [[1,2],[2,2]]
* Output: false
* Explanation:
* The diagonal "[1, 2]" has different elements.
*
*
* Constraints:
*
* * `m == matrix.length`
* * `n == matrix[i].length`
* * `1 <= m, n <= 20`
* * `0 <= matrix[i][j] <= 99`
*
*
* Follow up:
*
* * What if the `matrix` is stored on disk, and the memory is limited such that
*   you can only load at most one row of the matrix into the memory at once?
* * What if the `matrix` is so large that you can only load up a partial row into
*   the memory at once?
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
    pub fn is_toeplitz_matrix_at(matrix: &[Vec<i32>], i: usize, j: usize) -> bool {
        if i == matrix.len() - 1 || j == matrix[0].len() - 1 {
            return true;
        }
        matrix[i][j] == matrix[i + 1][j + 1] && Self::is_toeplitz_matrix_at(matrix, i, j + 1)
    }
    pub fn is_toeplitz_matrix(matrix: Vec<Vec<i32>>) -> bool {
        for i in 0..matrix.len() {
            if !Self::is_toeplitz_matrix_at(&matrix, i, 0) {
                return false;
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
        assert!(Solution::is_toeplitz_matrix(vec![
            vec![1, 2, 3, 4],
            vec![5, 1, 2, 3],
            vec![9, 5, 1, 2],
        ]))
    }

    #[test]
    fn example_2() {
        assert!(!Solution::is_toeplitz_matrix(vec![vec![1, 2], vec![2, 2]]))
    }
}
// @leetup=inject:after_code
