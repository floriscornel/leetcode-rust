// @leetup=custom
// @leetup=info id=74 lang=rust slug=search-a-2d-matrix

/*
* Write an efficient algorithm that searches for a value `target` in an `m x n`
* integer matrix `matrix`. This matrix has the following properties:
*
* * Integers in each row are sorted from left to right.
* * The first integer of each row is greater than the last integer of the previous
*   row.
*
*
* Example 1:
*
* []
* Input: matrix = [[1,3,5,7],[10,11,16,20],[23,30,34,60]], target = 3
* Output: true
*
* Example 2:
*
* []
* Input: matrix = [[1,3,5,7],[10,11,16,20],[23,30,34,60]], target = 13
* Output: false
*
*
* Constraints:
*
* * `m == matrix.length`
* * `n == matrix[i].length`
* * `1 <= m, n <= 100`
* * `-104 <= matrix[i][j], target <= 104`
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
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        // Matrix boundaries matrix[0 <= y < m][0 <= x < n]
        let (m, n) = (matrix.len(), matrix[0].len());
        // Start at top left of matrix
        let (mut y, mut x) = (0, 0);
        loop {
            // Each itteration we will either: return answer OR go down OR go right
            if matrix[y][x] == target {
                // we found our target
                return true;
            } else if y + 1 < m && matrix[y + 1][x] <= target {
                // if element down is smaller or equal to target, we should go down
                y += 1;
            } else if x + 1 < n && matrix[y][x + 1] <= target {
                // if element right is smaller or equal to target, we should go right
                x += 1;
            } else {
                // if there is no valid choice we are done
                return false;
            }
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
        assert!(Solution::search_matrix(
            vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
            3
        ));
    }

    #[test]
    fn example_2() {
        assert!(!Solution::search_matrix(
            vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
            13
        ));
    }
}
// @leetup=inject:after_code
