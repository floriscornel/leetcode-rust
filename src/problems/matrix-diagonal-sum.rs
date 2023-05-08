// @leetup=custom
// @leetup=info id=1572 lang=rust slug=matrix-diagonal-sum

/*
* Given a square matrix `mat`, return the sum of the matrix diagonals.
*
* Only include the sum of all the elements on the primary diagonal and all the
* elements on the secondary diagonal that are not part of the primary diagonal.
*
*
* Example 1:
*
* []
* Input: mat = [[1,2,3],
*               [4,5,6],
*               [7,8,9]]
* Output: 25
* Explanation: Diagonals sum: 1 + 5 + 9 + 3 + 7 = 25
* Notice that element mat[1][1] = 5 is counted only once.
*
* Example 2:
*
* Input: mat = [[1,1,1,1],
*               [1,1,1,1],
*               [1,1,1,1],
*               [1,1,1,1]]
* Output: 8
*
* Example 3:
*
* Input: mat = [[5]]
* Output: 5
*
*
* Constraints:
*
* * `n == mat.length == mat[i].length`
* * `1 <= n <= 100`
* * `1 <= mat[i][j] <= 100`
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
    pub fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
        mat.into_iter()
            .fold((0, 0), |(mut sum, mut i), row| {
                sum += row[i];
                if i != row.len() - 1 - i {
                    sum += row[row.len() - 1 - i];
                }
                i += 1;
                (sum, i)
            })
            .0
    }
}
// @leetup=code

// @leetup=inject:after_code
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let mat = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        assert_eq!(Solution::diagonal_sum(mat), 25);
    }

    #[test]
    fn example_2() {
        let mat = vec![
            vec![1, 1, 1, 1],
            vec![1, 1, 1, 1],
            vec![1, 1, 1, 1],
            vec![1, 1, 1, 1],
        ];
        assert_eq!(Solution::diagonal_sum(mat), 8);
    }

    #[test]
    fn example_3() {
        let mat = vec![vec![5]];
        assert_eq!(Solution::diagonal_sum(mat), 5);
    }
}
// @leetup=inject:after_code
