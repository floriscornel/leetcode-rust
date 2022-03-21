// @leetup=custom
// @leetup=info id=1007 lang=rust slug=minimum-domino-rotations-for-equal-row

/*
* In a row of dominoes, `tops[i]` and `bottoms[i]` represent the top and bottom
* halves of the `ith` domino. (A domino is a tile with two numbers from 1 to 6 -
* one on each half of the tile.)
*
* We may rotate the `ith` domino, so that `tops[i]` and `bottoms[i]` swap values.
*
* Return the minimum number of rotations so that all the values in `tops` are the
* same, or all the values in `bottoms` are the same.
*
* If it cannot be done, return `-1`.
*
*
* Example 1:
*
* []
* Input: tops = [2,1,2,4,2,2], bottoms = [5,2,6,2,3,2]
* Output: 2
* Explanation:
* The first figure represents the dominoes as given by tops and bottoms: before we
*  do any rotations.
* If we rotate the second and fourth dominoes, we can make every value in the top
* row equal to 2, as indicated by the second figure.
*
* Example 2:
*
* Input: tops = [3,5,1,2,3], bottoms = [3,6,3,3,4]
* Output: -1
* Explanation:
* In this case, it is not possible to rotate the dominoes to make one row of value
* s equal.
*
*
* Constraints:
*
* * `2 <= tops.length <= 2 * 104`
* * `bottoms.length == tops.length`
* * `1 <= tops[i], bottoms[i] <= 6`
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
    fn solve(a: &[i32], b: &[i32], n: i32) -> Option<i32> {
        let mut flips = 0;
        for index in 0..a.len() {
            if a[index] != n {
                if b[index] == n {
                    flips += 1;
                } else {
                    return None;
                }
            }
        }
        Some(flips)
    }

    pub fn min_domino_rotations(tops: Vec<i32>, bottoms: Vec<i32>) -> i32 {
        let mut solutions = Vec::new();
        if let Some(solution) = Solution::solve(&tops, &bottoms, tops[0]) {
            solutions.push(solution);
        }
        if let Some(solution) = Solution::solve(&bottoms, &tops, bottoms[0]) {
            solutions.push(solution);
        }
        if let Some(solution) = Solution::solve(&tops, &bottoms, bottoms[0]) {
            solutions.push(solution);
        }
        if let Some(solution) = Solution::solve(&bottoms, &tops, tops[0]) {
            solutions.push(solution);
        }

        *solutions.iter().min().unwrap_or(&-1)
    }
}
// @leetup=code

// @leetup=inject:after_code
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let tops = vec![2, 1, 2, 4, 2, 2];
        let bottoms = vec![5, 2, 6, 2, 3, 2];
        assert_eq!(Solution::min_domino_rotations(tops, bottoms), 2)
    }

    #[test]
    fn example_2() {
        let tops = vec![3, 5, 1, 2, 3];
        let bottoms = vec![3, 6, 3, 3, 4];
        assert_eq!(Solution::min_domino_rotations(tops, bottoms), -1)
    }

    #[test]
    fn example_3() {
        let tops = vec![1, 1, 1];
        let bottoms = vec![1, 1, 1];
        assert_eq!(Solution::min_domino_rotations(tops, bottoms), 0)
    }

    #[test]
    fn example_4() {
        let tops = vec![1, 2];
        let bottoms = vec![1, 2];
        assert_eq!(Solution::min_domino_rotations(tops, bottoms), -1)
    }

    #[test]
    fn example_5() {
        let tops = vec![1, 2];
        let bottoms = vec![2, 1];
        assert_eq!(Solution::min_domino_rotations(tops, bottoms), 1)
    }

    #[test]
    fn example_6() {
        let tops = vec![1, 3, 2, 3, 4, 1, 1, 4, 1];
        let bottoms = vec![5, 1, 5, 5, 2, 3, 5, 3, 1];
        assert_eq!(Solution::min_domino_rotations(tops, bottoms), -1)
    }

    #[test]
    fn example_7() {
        let tops = vec![
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 1, 1, 1, 1, 1, 2, 1, 2, 2, 1,
            1, 2, 2, 2, 2, 2, 1, 1, 2, 2, 2, 2, 1, 2, 1, 1, 2, 1, 1, 1, 1, 2, 1, 2, 2, 2, 1, 2, 1,
            2, 2, 1, 2, 1, 2,
        ];
        let bottoms = vec![
            2, 1, 1, 1, 2, 1, 2, 1, 2, 2, 1, 1, 1, 2, 1, 2, 2, 1, 2, 2, 2, 1, 2, 2, 1, 1, 1, 2, 1,
            2, 2, 1, 2, 1, 1, 2, 1, 1, 1, 2, 1, 2, 2, 2, 2, 1, 2, 1, 1, 2, 1, 2, 2, 1, 2, 1, 1, 2,
            2, 1, 2, 1, 1, 2,
        ];
        assert_eq!(Solution::min_domino_rotations(tops, bottoms), -1)
    }

    #[test]
    fn example_8() {
        let bottoms = vec![
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 1, 1, 1, 1, 1, 2, 1, 2, 2, 1,
            1, 2, 2, 2, 2, 2, 1, 1, 2, 2, 2, 2, 1, 2, 1, 1, 2, 1, 1, 1, 1, 2, 1, 2, 2, 2, 1, 2, 1,
            2, 2, 1, 2, 1, 2,
        ];
        let tops = vec![
            2, 1, 1, 1, 2, 1, 2, 1, 2, 2, 1, 1, 1, 2, 1, 2, 2, 1, 2, 2, 2, 1, 2, 2, 1, 1, 1, 2, 1,
            2, 2, 1, 2, 1, 1, 2, 1, 1, 1, 2, 1, 2, 2, 2, 2, 1, 2, 1, 1, 2, 1, 2, 2, 1, 2, 1, 1, 2,
            2, 1, 2, 1, 1, 2,
        ];
        assert_eq!(Solution::min_domino_rotations(tops, bottoms), -1)
    }

    #[test]
    fn example_9() {
        let tops = vec![1, 1, 1, 2, 2];
        let bottoms = vec![2, 2, 2, 1, 3];
        assert_eq!(Solution::min_domino_rotations(tops, bottoms), 2)
    }
}
// @leetup=inject:after_code
