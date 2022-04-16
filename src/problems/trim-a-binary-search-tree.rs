// @leetup=custom
// @leetup=info id=669 lang=rust slug=trim-a-binary-search-tree

/*
* Given the `root` of a binary search tree and the lowest and highest boundaries
* as `low` and `high`, trim the tree so that all its elements lies in `[low,
* high]`. Trimming the tree should not change the relative structure of the
* elements that will remain in the tree (i.e., any node's descendant should remain
* a descendant). It can be proven that there is a unique answer.
*
* Return *the root of the trimmed binary search tree*. Note that the root may
* change depending on the given bounds.
*
*
* Example 1:
*
* []
* Input: root = [1,0,2], low = 1, high = 2
* Output: [1,null,2]
*
* Example 2:
*
* []
* Input: root = [3,0,4,null,2,null,null,1], low = 1, high = 3
* Output: [3,2,null,1]
*
*
* Constraints:
*
* * The number of nodes in the tree in the range `[1, 104]`.
* * `0 <= Node.val <= 104`
* * The value of each node in the tree is unique.
* * `root` is guaranteed to be a valid binary search tree.
* * `0 <= low <= high <= 104`
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
    pub fn trim_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        low: i32,
        high: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(root_ref) = root.as_ref() {
            let mut node = root_ref.borrow_mut();
            if node.val < low {
                return Solution::trim_bst(node.right.take(), low, high);
            } else if node.val > high {
                return Solution::trim_bst(node.left.take(), low, high);
            }
            node.left = Solution::trim_bst(node.left.take(), low, high);
            node.right = Solution::trim_bst(node.right.take(), low, high);
        }
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
        assert_eq!(Solution::trim_bst(tree![1, 0, 2], 1, 2), tree![1, null, 2]);
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::trim_bst(tree![3, 0, 4, null, 2, null, null, 1], 1, 3),
            tree![3, 2, null, 1]
        );
    }
}
// @leetup=inject:after_code
