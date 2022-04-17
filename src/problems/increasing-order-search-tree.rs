// @leetup=custom
// @leetup=info id=897 lang=rust slug=increasing-order-search-tree

/*
* Given the `root` of a binary search tree, rearrange the tree in in-order so
* that the leftmost node in the tree is now the root of the tree, and every node
* has no left child and only one right child.
*
*
* Example 1:
*
* []
* Input: root = [5,3,6,2,4,null,8,1,null,null,null,7,9]
* Output: [1,null,2,null,3,null,4,null,5,null,6,null,7,null,8,null,9]
*
* Example 2:
*
* []
* Input: root = [5,1,7]
* Output: [1,null,5,null,7]
*
*
* Constraints:
*
* * The number of nodes in the given tree will be in the range `[1, 100]`.
* * `0 <= Node.val <= 1000`
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
    pub fn increasing_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut values = Vec::<i32>::new();
        fn visit_inorder(root: &Option<Rc<RefCell<TreeNode>>>, values: &mut Vec<i32>) {
            if let Some(node) = root {
                visit_inorder(&node.borrow().left, values);
                values.push(node.borrow().val);
                visit_inorder(&node.borrow().right, values);
            }
        }
        visit_inorder(&root, &mut values);

        let mut tree_node: Option<Rc<RefCell<TreeNode>>> = None;
        while let Some(val) = values.pop() {
            tree_node = Some(Rc::new(RefCell::new(TreeNode {
                val,
                left: None,
                right: tree_node,
            })));
        }
        tree_node
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
            Solution::increasing_bst(tree![5, 3, 6, 2, 4, null, 8, 1, null, null, null, 7, 9]),
            tree![1, null, 2, null, 3, null, 4, null, 5, null, 6, null, 7, null, 8, null, 9]
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::increasing_bst(tree![5, 1, 7]),
            tree![1, null, 5, null, 7]
        );
    }
}
// @leetup=inject:after_code
