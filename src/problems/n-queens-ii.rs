// @leetup=custom
// @leetup=info id=52 lang=rust slug=n-queens-ii

/*
* The n-queens puzzle is the problem of placing `n` queens on an `n x n`
* chessboard such that no two queens attack each other.
*
* Given an integer `n`, return *the number of distinct solutions to the n-queens
* puzzle*.
*
*
* Example 1:
*
* []
* Input: n = 4
* Output: 2
* Explanation: There are two distinct solutions to the 4-queens puzzle as show
* n.
*
* Example 2:
*
* Input: n = 1
* Output: 1
*
*
* Constraints:
*
* * `1 <= n <= 9`
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
    fn inner_recurse(
        left_diagonal: i64,
        column: i64,
        right_diagonal: i64,
        done: i64,
        count: &mut i32,
    ) {
        if column == done {
            *count += 1;
            return;
        }
        let mut poss = !(left_diagonal | right_diagonal | column);
        while poss & done > 0 {
            let bit = poss & -poss;
            poss -= bit;
            Solution::inner_recurse(
                (left_diagonal | bit) >> 1,
                column | bit,
                (right_diagonal | bit) << 1,
                done,
                count,
            );
        }
    }

    pub fn total_n_queens(n: i32) -> i32 {
        let mut count = 0;
        let done = i64::pow(2, n as u32) - 1;
        let inner_recurse = Solution::inner_recurse(0, 0, 0, done, &mut count);
        count
    }
}
// @leetup=code

// @leetup=inject:after_code
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        assert_eq!(Solution::total_n_queens(4), 2);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::total_n_queens(1), 1);
    }
}
// @leetup=inject:after_code
