// @leetup=custom
// @leetup=info id=222 lang=rust slug=count-complete-tree-nodes

/*
* Given the `root` of a complete binary tree, return the number of the nodes
* in the tree.
*
* According to [Wikipedia][1], every level, except possibly the last, is
* completely filled in a complete binary tree, and all nodes in the last level are
* as far left as possible. It can have between `1` and `2h` nodes inclusive at the
* last level `h`.
*
* Design an algorithm that runs in less than `O(n)` time complexity.
*
*
* Example 1:
*
* []
* Input: root = [1,2,3,4,5,6]
* Output: 6
*
* Example 2:
*
* Input: root = []
* Output: 0
*
* Example 3:
*
* Input: root = [1]
* Output: 1
*
*
* Constraints:
*
* * The number of nodes in the tree is in the range `[0, 5 * 104]`.
* * `0 <= Node.val <= 5 * 104`
* * The tree is guaranteed to be complete.
*
* [1] http://en.wikipedia.org/wiki/Binary_tree#Types_of_binary_trees
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
type Node = Option<Rc<RefCell<TreeNode>>>;
impl Solution {
    pub fn count(node: &Node) -> i32 {
        if let Some(node) = node {
            let x = node.borrow();
            1 + Self::count(&x.left) + Self::count(&x.right)
        } else {
            0
        }
    }

    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::count(&root)
    }
}
// @leetup=code

// @leetup=inject:after_code
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        assert_eq!(Solution::count_nodes(tree![1, 2, 3, 4, 5, 6]), 6);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::count_nodes(tree![]), 0);
    }

    #[test]
    fn example_3() {
        assert_eq!(Solution::count_nodes(tree![1]), 1);
    }
}
// @leetup=inject:after_code
