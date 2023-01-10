// @leetup=custom
// @leetup=info id=100 lang=rust slug=same-tree

/*
* Given the roots of two binary trees `p` and `q`, write a function to check if
* they are the same or not.
*
* Two binary trees are considered the same if they are structurally identical, and
* the nodes have the same value.
*
*
* Example 1:
*
* []
* Input: p = [1,2,3], q = [1,2,3]
* Output: true
*
* Example 2:
*
* []
* Input: p = [1,2], q = [1,null,2]
* Output: false
*
* Example 3:
*
* []
* Input: p = [1,2,1], q = [1,1,2]
* Output: false
*
*
* Constraints:
*
* * The number of nodes in both trees is in the range `[0, 100]`.
* * `-104 <= Node.val <= 104`
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
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (p, q) {
            (None, None) => true,
            (Some(p), Some(q)) => {
                let p = p.borrow();
                let q = q.borrow();
                p.val == q.val
                    && Self::is_same_tree(p.left.clone(), q.left.clone())
                    && Self::is_same_tree(p.right.clone(), q.right.clone())
            }
            _ => false,
        }
    }
}
// @leetup=code

// @leetup=inject:after_code
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let p = tree!(1, 2, 3);
        let q = tree!(1, 2, 3);
        let expected = true;
        assert_eq!(Solution::is_same_tree(p, q), expected);
    }

    #[test]
    fn example_2() {
        let p = tree!(1, 2);
        let q = tree!(1, null, 2);
        let expected = false;
        assert_eq!(Solution::is_same_tree(p, q), expected);
    }

    #[test]
    fn example_3() {
        let p = tree!(1, 2, 1);
        let q = tree!(1, 1, 2);
        let expected = false;
        assert_eq!(Solution::is_same_tree(p, q), expected);
    }
}
// @leetup=inject:after_code
