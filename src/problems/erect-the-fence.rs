// @leetup=custom
// @leetup=info id=587 lang=rust slug=erect-the-fence

/*
* You are given an array `trees` where `trees[i] = [xi, yi]` represents the
* location of a tree in the garden.
*
* You are asked to fence the entire garden using the minimum length of rope as it
* is expensive. The garden is well fenced only if all the trees are enclosed.
*
* Return *the coordinates of trees that are exactly located on the fence
* perimeter*.
*
*
* Example 1:
*
* []
* Input: points = [[1,1],[2,2],[2,0],[2,4],[3,3],[4,2]]
* Output: [[1,1],[2,0],[3,3],[2,4],[4,2]]
*
* Example 2:
*
* []
* Input: points = [[1,2],[2,2],[4,2]]
* Output: [[4,2],[2,2],[1,2]]
*
*
* Constraints:
*
* * `1 <= points.length <= 3000`
* * `points[i].length == 2`
* * `0 <= xi, yi <= 100`
* * All the given points are unique.
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
    pub fn outer_trees(mut trees: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = trees.len();
        let mut ret: Vec<Vec<i32>> = vec![];

        trees.sort();

        for tree in trees.iter() {
            while ret.len() > 1 {
                if Self::orientation(&ret[ret.len() - 2], &ret[ret.len() - 1], tree) {
                    break;
                }
                ret.pop();
            }
            ret.push(tree.clone());
        }

        if ret.len() == n {
            return ret;
        }

        for i in (0..n - 1).rev() {
            while ret.len() > 1 {
                if Self::orientation(&ret[ret.len() - 2], &ret[ret.len() - 1], &trees[i]) {
                    break;
                }
                ret.pop();
            }
            ret.push(trees[i].clone());
        }

        ret.pop();

        ret
    }

    fn orientation(a: &[i32], b: &[i32], c: &[i32]) -> bool {
        (b[0] - a[0]) * (c[1] - b[1]) - (b[1] - a[1]) * (c[0] - b[0]) >= 0
    }
}
// @leetup=code

// @leetup=inject:after_code
#[cfg(test)]
mod tests {
    use std::{collections::HashSet, iter::FromIterator};

    use super::Solution;

    #[test]
    fn example_1() {
        let trees = vec![
            vec![1, 1],
            vec![2, 2],
            vec![2, 0],
            vec![2, 4],
            vec![3, 3],
            vec![4, 2],
        ];
        let output = vec![vec![1, 1], vec![2, 0], vec![3, 3], vec![2, 4], vec![4, 2]];
        assert_eq!(
            HashSet::<Vec<i32>>::from_iter(Solution::outer_trees(trees).into_iter()),
            HashSet::<Vec<i32>>::from_iter(output.into_iter())
        );
    }

    #[test]
    fn example_2() {
        let trees = vec![vec![1, 2], vec![2, 2], vec![4, 2]];
        let output = vec![vec![4, 2], vec![2, 2], vec![1, 2]];
        assert_eq!(
            HashSet::<Vec<i32>>::from_iter(Solution::outer_trees(trees).into_iter()),
            HashSet::<Vec<i32>>::from_iter(output.into_iter())
        );
    }
}
// @leetup=inject:after_code
