// @leetup=custom
// @leetup=info id=1026 lang=rust slug=maximum-difference-between-node-and-ancestor

/*
* Given the `root` of a binary tree, find the maximum value `v` for which there
* exist different nodes `a` and `b` where `v = |a.val - b.val|` and `a` is an
* ancestor of `b`.
*
* A node `a` is an ancestor of `b` if either: any child of `a` is equal to `b` or
* any child of `a` is an ancestor of `b`.
*
*
* Example 1:
*
* []
* Input: root = [8,3,10,1,6,null,14,null,null,4,7,13]
* Output: 7
* Explanation: We have various ancestor-node differences, some of which are gi
* ven below :
* |8 - 3| = 5
* |3 - 7| = 4
* |8 - 1| = 7
* |10 - 13| = 3
* Among all possible differences, the maximum value of 7 is obtained by |8 - 1| =
* 7.
*
* Example 2:
*
* []
* Input: root = [1,null,2,null,0,3]
* Output: 3
*
*
* Constraints:
*
* * The number of nodes in the tree is in the range `[2, 5000]`.
* * `0 <= Node.val <= 105`
*
*/
// @leetup=custom
// @leetup=inject:before_code_ex
// before_code_exclude
#![allow(dead_code, unused_variables)]
struct Solution {}
use crate::util::tree::TreeNode;
// @leetup=inject:before_code_ex

// @leetup=code

// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn max_ancestor_diff(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, min: i32, max: i32) -> i32 {
            if let Some(node) = node {
                let node = node.borrow();
                let (min, max) = (min.min(node.val), max.max(node.val));
                let (left, right) = (dfs(&node.left, min, max), dfs(&node.right, min, max));
                (max - min).max(left).max(right)
            } else {
                0
            }
        }
        dfs(&root, i32::MAX, i32::MIN)
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
            Solution::max_ancestor_diff(tree![8, 3, 10, 1, 6, null, 14, null, null, 4, 7, 13]),
            7
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::max_ancestor_diff(tree![1, null, 2, null, 0, 3]),
            3
        );
    }
}
// @leetup=inject:after_code
