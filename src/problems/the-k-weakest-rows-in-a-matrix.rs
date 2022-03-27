// @leetup=custom
// @leetup=info id=1337 lang=rust slug=the-k-weakest-rows-in-a-matrix

/*
* You are given an `m x n` binary matrix `mat` of `1`'s (representing soldiers)
* and `0`'s (representing civilians). The soldiers are positioned in front of
* the civilians. That is, all the `1`'s will appear to the left of all the
* `0`'s in each row.
*
* A row `i` is weaker than a row `j` if one of the following is true:
*
* * The number of soldiers in row `i` is less than the number of soldiers in row
*   `j`.
* * Both rows have the same number of soldiers and `i < j`.
*
* Return *the indices of the *`k`* weakest rows in the matrix ordered from
* weakest to strongest*.
*
*
* Example 1:
*
* Input: mat =
* [[1,1,0,0,0],
*  [1,1,1,1,0],
*  [1,0,0,0,0],
*  [1,1,0,0,0],
*  [1,1,1,1,1]],
* k = 3
* Output: [2,0,3]
* Explanation:
* The number of soldiers in each row is:
* - Row 0: 2
* - Row 1: 4
* - Row 2: 1
* - Row 3: 2
* - Row 4: 5
* The rows ordered from weakest to strongest are [2,0,3,1,4].
*
* Example 2:
*
* Input: mat =
* [[1,0,0,0],
*  [1,1,1,1],
*  [1,0,0,0],
*  [1,0,0,0]],
* k = 2
* Output: [0,2]
* Explanation:
* The number of soldiers in each row is:
* - Row 0: 1
* - Row 1: 4
* - Row 2: 1
* - Row 3: 1
* The rows ordered from weakest to strongest are [0,2,3,1].
*
*
* Constraints:
*
* * `m == mat.length`
* * `n == mat[i].length`
* * `2 <= n, m <= 100`
* * `1 <= k <= m`
* * `matrix[i][j]` is either 0 or 1.
*
*/
// @leetup=custom
// @leetup=inject:before_code_ex
// before_code_exclude
#![allow(dead_code, unused_variables)]
struct Solution {}
// @leetup=inject:before_code_ex

// @leetup=code
use std::cmp::Ordering;

impl Solution {
    pub fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let mut index_sum_pairs: Vec<(usize, i32)> =
            mat.iter().map(|row| row.iter().sum()).enumerate().collect();

        index_sum_pairs.sort_unstable_by(|(index_a, a), (index_b, b)| {
            if a < b {
                Ordering::Less
            } else if a > b {
                Ordering::Greater
            } else if index_a < index_b {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });

        index_sum_pairs
            .drain(0..(k as usize))
            .map(|(index, value)| index as i32)
            .collect()
    }
}
// @leetup=code

// @leetup=inject:after_code
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let mat = vec![
            vec![1, 1, 0, 0, 0],
            vec![1, 1, 1, 1, 0],
            vec![1, 0, 0, 0, 0],
            vec![1, 1, 0, 0, 0],
            vec![1, 1, 1, 1, 1],
        ];
        let k = 3;
        assert_eq!(Solution::k_weakest_rows(mat, k), vec![2, 0, 3]);
    }

    #[test]
    fn example_2() {
        let mat = vec![
            vec![1, 0, 0, 0],
            vec![1, 1, 1, 1],
            vec![1, 0, 0, 0],
            vec![1, 0, 0, 0],
        ];
        let k = 2;
        assert_eq!(Solution::k_weakest_rows(mat, k), vec![0, 2]);
    }
}
// @leetup=inject:after_code
