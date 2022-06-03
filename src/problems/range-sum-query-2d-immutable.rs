// @leetup=custom
// @leetup=info id=304 lang=rust slug=range-sum-query-2d-immutable

/*
* Given a 2D matrix `matrix`, handle multiple queries of the following type:
*
* * Calculate the sum of the elements of `matrix` inside the rectangle defined
*   by its upper left corner `(row1, col1)` and lower right corner `(row2,
*   col2)`.
*
* Implement the NumMatrix class:
*
* * `NumMatrix(int[][] matrix)` Initializes the object with the integer matrix
*   `matrix`.
* * `int sumRegion(int row1, int col1, int row2, int col2)` Returns the sum of
*   the elements of `matrix` inside the rectangle defined by its upper left
*   corner `(row1, col1)` and lower right corner `(row2, col2)`.
*
*
* Example 1:
*
* []
* Input
* ["NumMatrix", "sumRegion", "sumRegion", "sumRegion"]
* [[[[3, 0, 1, 4, 2], [5, 6, 3, 2, 1], [1, 2, 0, 1, 5], [4, 1, 0, 1, 7], [1, 0, 3,
*  0, 5]]], [2, 1, 4, 3], [1, 1, 2, 2], [1, 2, 2, 4]]
* Output
* [null, 8, 11, 12]
* Explanation
* NumMatrix numMatrix = new NumMatrix([[3, 0, 1, 4, 2], [5, 6, 3, 2, 1], [1, 2, 0,
*  1, 5], [4, 1, 0, 1, 7], [1, 0, 3, 0, 5]]);
* numMatrix.sumRegion(2, 1, 4, 3); // return 8 (i.e sum of the red rectangle)
* numMatrix.sumRegion(1, 1, 2, 2); // return 11 (i.e sum of the green rectangle)
* numMatrix.sumRegion(1, 2, 2, 4); // return 12 (i.e sum of the blue rectangle)
*
*
* Constraints:
*
* * `m == matrix.length`
* * `n == matrix[i].length`
* * `1 <= m, n <= 200`
* * `-105 <= matrix[i][j] <= 105`
* * `0 <= row1 <= row2 < m`
* * `0 <= col1 <= col2 < n`
* * At most `104` calls will be made to `sumRegion`.
*
*/
// @leetup=custom
// @leetup=inject:before_code_ex
// before_code_exclude
#![allow(dead_code, unused_variables)]
struct Solution {}
// @leetup=inject:before_code_ex

// @leetup=code

struct NumMatrix {
    sums: Vec<Vec<i32>>,
}

impl NumMatrix {
    fn new(matrix: Vec<Vec<i32>>) -> Self {
        let (n, m) = (matrix.len(), matrix[0].len());
        let mut sums = vec![vec![0; m + 1]; n + 1];
        sums[0][0] = matrix[0][0];
        for i in 1..=n {
            for j in 1..=m {
                sums[i][j] =
                    matrix[i - 1][j - 1] + sums[i - 1][j] + sums[i][j - 1] - sums[i - 1][j - 1];
            }
        }

        Self { sums }
    }

    fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        self.sums[(row2 + 1) as usize][(col2 + 1) as usize]
            - self.sums[row1 as usize][(col2 + 1) as usize]
            - self.sums[(row2 + 1) as usize][col1 as usize]
            + self.sums[row1 as usize][col1 as usize]
    }
}

/**
 * Your NumMatrix object will be instantiated and called as such:
 * let obj = NumMatrix::new(matrix);
 * let ret_1: i32 = obj.sum_region(row1, col1, row2, col2);
 */
// @leetup=code

// @leetup=inject:after_code
#[cfg(test)]
mod tests {
    use super::NumMatrix;

    #[test]
    fn example_1() {
        let matrix = NumMatrix::new(vec![
            vec![3, 0, 1, 4, 2],
            vec![5, 6, 3, 2, 1],
            vec![1, 2, 0, 1, 5],
            vec![4, 1, 0, 1, 7],
            vec![1, 0, 3, 0, 5],
        ]);
        assert_eq!(matrix.sum_region(2, 1, 4, 3), 8);
        assert_eq!(matrix.sum_region(1, 1, 2, 2), 11);
        assert_eq!(matrix.sum_region(1, 2, 2, 4), 12);
    }

    #[test]
    fn example_2() {
        let matrix = NumMatrix::new(vec![vec![-4, -5]]);
        assert_eq!(matrix.sum_region(0, 0, 0, 0), -4);
        assert_eq!(matrix.sum_region(0, 0, 0, 1), -9);
        assert_eq!(matrix.sum_region(0, 1, 0, 1), -5);
    }

    #[test]
    fn example_3() {
        let matrix = NumMatrix::new(vec![vec![-4], vec![-5]]);
        assert_eq!(matrix.sum_region(0, 0, 0, 0), -4);
        assert_eq!(matrix.sum_region(0, 0, 1, 0), -9);
        assert_eq!(matrix.sum_region(1, 0, 1, 0), -5);
    }
}
// @leetup=inject:after_code
