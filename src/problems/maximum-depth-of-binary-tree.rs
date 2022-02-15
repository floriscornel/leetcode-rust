// @leetup=custom
// @leetup=info id=104 lang=rust slug=maximum-depth-of-binary-tree

/*
* Given the `root` of a binary tree, return *its maximum depth*.
*
* A binary tree's maximum depth is the number of nodes along the longest path
* from the root node down to the farthest leaf node.
*
*
* Example 1:
*
* []
* Input: root = [3,9,20,null,null,15,7]
* Output: 3
*
* Example 2:
*
* Input: root = [1,null,2]
* Output: 2
*
*
* Constraints:
*
* * The number of nodes in the tree is in the range `[0, 104]`.
* * `-100 <= Node.val <= 100`
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
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn helper(root: Option<&Rc<RefCell<TreeNode>>>) -> i32 {
            match root {
                Some(node) => {
                    1 + std::cmp::max(
                        helper(node.borrow().left.as_ref()),
                        helper(node.borrow().right.as_ref()),
                    )
                }
                None => 0,
            }
        }
        helper(root.as_ref())
    }
}
// @leetup=code

// @leetup=inject:after_code
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let input = tree![3, 9, 20, null, null, 15, 7];
        let output = 3;
        assert_eq!(Solution::max_depth(input), output);
    }

    #[test]
    fn example_2() {
        let input = tree![1, null, 2];
        let output = 2;
        assert_eq!(Solution::max_depth(input), output);
    }
}
// @leetup=inject:after_code
