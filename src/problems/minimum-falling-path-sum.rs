// @leetup=custom
// @leetup=info id=931 lang=rust slug=minimum-falling-path-sum

/*
* Given an `n x n` array of integers `matrix`, return *the minimum sum of any
* falling path through* `matrix`.
*
* A falling path starts at any element in the first row and chooses the
* element in the next row that is either directly below or diagonally left/right.
* Specifically, the next element from position `(row, col)` will be `(row + 1, col
* - 1)`, `(row + 1, col)`, or `(row + 1, col + 1)`.
*
*
* Example 1:
*
* []
* Input: matrix = [[2,1,3],[6,5,4],[7,8,9]]
* Output: 13
* Explanation: There are two falling paths with a minimum sum as shown.
*
* Example 2:
*
* []
* Input: matrix = [[-19,57],[-40,-5]]
* Output: -59
* Explanation: The falling path with a minimum sum is shown.
*
*
* Constraints:
*
* * `n == matrix.length == matrix[i].length`
* * `1 <= n <= 100`
* * `-100 <= matrix[i][j] <= 100`
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
    pub fn min_falling_path_sum(matrix: Vec<Vec<i32>>) -> i32 {
        let n = matrix.len();

        let adjecent = |x: usize| match x {
            0 => vec![0, 1],
            x if x == n - 1 => vec![x - 1, x],
            x => vec![x - 1, x, x + 1],
        };

        let mut dp = vec![vec![0; n]; n];
        for i in 0..n {
            dp[0][i] = matrix[0][i];
        }

        for i in 1..n {
            for j in 0..n {
                dp[i][j] = adjecent(j)
                    .iter()
                    .map(|&x| dp[i - 1][x] + matrix[i][j])
                    .min()
                    .unwrap();
            }
        }

        *dp[n - 1].iter().min().unwrap()
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
            Solution::min_falling_path_sum(matrix![[2, 1, 3], [6, 5, 4], [7, 8, 9]]),
            13
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::min_falling_path_sum(matrix![[-19, 57], [-40, -5]]),
            -59
        );
    }
}
// @leetup=inject:after_code
