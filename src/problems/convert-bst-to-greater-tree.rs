// @leetup=custom
// @leetup=info id=538 lang=rust slug=convert-bst-to-greater-tree

/*
* Given the `root` of a Binary Search Tree (BST), convert it to a Greater Tree
* such that every key of the original BST is changed to the original key plus the
* sum of all keys greater than the original key in BST.
*
* As a reminder, a *binary search tree* is a tree that satisfies these
* constraints:
*
* * The left subtree of a node contains only nodes with keys less than the
*   node's key.
* * The right subtree of a node contains only nodes with keys greater than the
*   node's key.
* * Both the left and right subtrees must also be binary search trees.
*
*
* Example 1:
*
* []
* Input: root = [4,1,6,0,2,5,7,null,null,null,3,null,null,null,8]
* Output: [30,36,21,36,35,26,15,null,null,null,33,null,null,null,8]
*
* Example 2:
*
* Input: root = [0,null,1]
* Output: [1,null,1]
*
*
* Constraints:
*
* * The number of nodes in the tree is in the range `[0, 104]`.
* * `-104 <= Node.val <= 104`
* * All the values in the tree are unique.
* * `root` is guaranteed to be a valid binary search tree.
*
*
* Note: This question is the same as 1038:
* [https://leetcode.com/problems/binary-search-tree-to-greater-sum-tree/][1]
*
* [1] https://leetcode.com/problems/binary-search-tree-to-greater-sum-tree/
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
    pub fn convert_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        fn traverse(root: &Option<Rc<RefCell<TreeNode>>>, sum: &mut i32) {
            if let Some(root_ref) = root.as_ref() {
                let mut node = root_ref.borrow_mut();
                traverse(&node.right, sum);
                *sum += node.val;
                node.val = *sum;
                traverse(&node.left, sum);
            }
        }
        traverse(&root, &mut 0);
        root
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
            Solution::convert_bst(tree![
                4, 1, 6, 0, 2, 5, 7, null, null, null, 3, null, null, null, 8
            ]),
            tree![30, 36, 21, 36, 35, 26, 15, null, null, null, 33, null, null, null, 8]
        )
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::convert_bst(tree![0, null, 1]), tree![1, null, 1])
    }
}
// @leetup=inject:after_code
