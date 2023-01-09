// @leetup=custom
// @leetup=info id=144 lang=rust slug=binary-tree-preorder-traversal

/*
* Given the `root` of a binary tree, return *the preorder traversal of its nodes'
* values*.
*
*
* Example 1:
*
* []
* Input: root = [1,null,2,3]
* Output: [1,2,3]
*
* Example 2:
*
* Input: root = []
* Output: []
*
* Example 3:
*
* Input: root = [1]
* Output: [1]
*
*
* Constraints:
*
* * The number of nodes in the tree is in the range `[0, 100]`.
* * `-100 <= Node.val <= 100`
*
*
* Follow up: Recursive solution is trivial, could you do it iteratively?
*
*/
// @leetup=custom
// @leetup=inject:before_code_ex
// before_code_exclude
#![allow(dead_code, unused_variables)]
struct Solution {}
// @leetup=inject:before_code_ex
use crate::util::tree::TreeNode;
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
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut values = Vec::<i32>::new();
        fn visit_preorder(root: &Option<Rc<RefCell<TreeNode>>>, values: &mut Vec<i32>) {
            if let Some(node) = root {
                values.push(node.borrow().val);
                visit_preorder(&node.borrow().left, values);
                visit_preorder(&node.borrow().right, values);
            }
        }
        visit_preorder(&root, &mut values);
        values
    }
}
// @leetup=code

// @leetup=inject:after_code
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let input = tree![1, null, 2, 3];
        let expected = vec![1, 2, 3];
        assert_eq!(Solution::preorder_traversal(input), expected);
    }

    #[test]
    fn example_2() {
        let input = tree![];
        let expected = vec![];
        assert_eq!(Solution::preorder_traversal(input), expected);
    }

    #[test]
    fn example_3() {
        let input = tree![1];
        let expected = vec![1];
        assert_eq!(Solution::preorder_traversal(input), expected);
    }
}
// @leetup=inject:after_code
