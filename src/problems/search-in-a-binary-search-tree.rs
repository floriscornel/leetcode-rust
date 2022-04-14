// @leetup=custom
// @leetup=info id=700 lang=rust slug=search-in-a-binary-search-tree

/*
* You are given the `root` of a binary search tree (BST) and an integer `val`.
*
* Find the node in the BST that the node's value equals `val` and return the
* subtree rooted with that node. If such a node does not exist, return `null`.
*
*
* Example 1:
*
* []
* Input: root = [4,2,7,1,3], val = 2
* Output: [2,1,3]
*
* Example 2:
*
* []
* Input: root = [4,2,7,1,3], val = 5
* Output: []
*
*
* Constraints:
*
* * The number of nodes in the tree is in the range `[1, 5000]`.
* * `1 <= Node.val <= 107`
* * `root` is a binary search tree.
* * `1 <= val <= 107`
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
    pub fn search_bst(
        mut root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        while let Some(node) = root {
            let current = node.borrow().val;
            match current.cmp(&val) {
                std::cmp::Ordering::Less => root = node.borrow().right.clone(),
                std::cmp::Ordering::Equal => return Some(node),
                std::cmp::Ordering::Greater => root = node.borrow().left.clone(),
            }
        }
        None
    }
}
// @leetup=code

// @leetup=inject:after_code
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let root = tree![4, 2, 7, 1, 3];
        let val = 2;
        let output = tree![2, 1, 3];
        assert_eq!(Solution::search_bst(root, val), output);
    }

    #[test]
    fn example_2() {
        let root = tree![4, 2, 7, 1, 3];
        let val = 5;
        let output = None;
        assert_eq!(Solution::search_bst(root, val), output);
    }
}
// @leetup=inject:after_code
