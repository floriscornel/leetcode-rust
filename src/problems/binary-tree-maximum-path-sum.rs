// @leetup=custom
// @leetup=info id=124 lang=rust slug=binary-tree-maximum-path-sum

/*
* A path in a binary tree is a sequence of nodes where each pair of adjacent
* nodes in the sequence has an edge connecting them. A node can only appear in the
* sequence at most once. Note that the path does not need to pass through the
* root.
*
* The path sum of a path is the sum of the node's values in the path.
*
* Given the `root` of a binary tree, return *the maximum path sum of any
* non-empty path*.
*
*
* Example 1:
*
* []
* Input: root = [1,2,3]
* Output: 6
* Explanation: The optimal path is 2 -> 1 -> 3 with a path sum of 2 + 1 + 3 =
* 6.
*
* Example 2:
*
* []
* Input: root = [-10,9,20,null,null,15,7]
* Output: 42
* Explanation: The optimal path is 15 -> 20 -> 7 with a path sum of 15 + 20 +
* 7 = 42.
*
*
* Constraints:
*
* * The number of nodes in the tree is in the range `[1, 3 * 104]`.
* * `-1000 <= Node.val <= 1000`
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
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, max_sum: &mut i32) -> i32 {
            if let Some(node) = node {
                let node = node.borrow();
                let left = dfs(&node.left, max_sum);
                let right = dfs(&node.right, max_sum);
                let sum = node.val + left.max(0) + right.max(0);
                *max_sum = (*max_sum).max(sum);
                node.val + left.max(0).max(right.max(0))
            } else {
                0
            }
        }

        let mut max_sum = std::i32::MIN;
        dfs(&root, &mut max_sum);
        max_sum
    }
}
// @leetup=code

// @leetup=inject:after_code
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        assert_eq!(Solution::max_path_sum(tree![1, 2, 3]), 6);
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::max_path_sum(tree![-10, 9, 20, null, null, 15, 7]),
            42
        );
    }
}
// @leetup=inject:after_code
