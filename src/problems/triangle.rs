// @leetup=custom
// @leetup=info id=120 lang=rust slug=triangle

/*
* Given a `triangle` array, return *the minimum path sum from top to bottom*.
*
* For each step, you may move to an adjacent number of the row below. More
* formally, if you are on index `i` on the current row, you may move to either
* index `i` or index `i + 1` on the next row.
*
*
* Example 1:
*
* Input: triangle = [[2],[3,4],[6,5,7],[4,1,8,3]]
* Output: 11
* Explanation: The triangle looks like:
*    2
*   3 4
*  6 5 7
* 4 1 8 3
* The minimum path sum from top to bottom is 2 + 3 + 5 + 1 = 11 (underlined above)
* .
*
* Example 2:
*
* Input: triangle = [[-10]]
* Output: -10
*
*
* Constraints:
*
* * `1 <= triangle.length <= 200`
* * `triangle[0].length == 1`
* * `triangle[i].length == triangle[i - 1].length + 1`
* * `-104 <= triangle[i][j] <= 104`
*
*
* Follow up: Could you do this using only `O(n)` extra space, where `n` is the
* total number of rows in the triangle?
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
    fn get_row_minimum(triangle: &mut [Vec<i32>], bottom_index: usize) {
        let top_size = triangle[bottom_index - 1].len();
        triangle[bottom_index][0] += triangle[bottom_index - 1][0];
        for i in 1..top_size {
            triangle[bottom_index][i] += i32::min(
                triangle[bottom_index - 1][i - 1],
                triangle[bottom_index - 1][i],
            );
        }
        triangle[bottom_index][top_size] += triangle[bottom_index - 1][top_size - 1];
    }

    pub fn minimum_total(mut triangle: Vec<Vec<i32>>) -> i32 {
        for bottom_index in 1..triangle.len() {
            Self::get_row_minimum(&mut triangle, bottom_index);
        }
        *triangle.last().unwrap().iter().min().unwrap()
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
            Solution::minimum_total(vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]]),
            11
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::minimum_total(vec![vec![-10]]), -10);
    }
}
// @leetup=inject:after_code
