// @leetup=custom
// @leetup=info id=872 lang=rust slug=leaf-similar-trees

/*
* Consider all the leaves of a binary tree, from left to right order, the values
* of those leaves form a leaf value sequence*.*
*
* []
*
* For example, in the given tree above, the leaf value sequence is `(6, 7, 4, 9,
* 8)`.
*
* Two binary trees are considered *leaf-similar* if their leaf value sequence is
* the same.
*
* Return `true` if and only if the two given trees with head nodes `root1` and
* `root2` are leaf-similar.
*
*
* Example 1:
*
* []
* Input: root1 = [3,5,1,6,2,9,8,null,null,7,4], root2 = [3,5,1,6,7,4,2,null,nu
* ll,null,null,null,null,9,8]
* Output: true
*
* Example 2:
*
* []
* Input: root1 = [1,2,3], root2 = [1,3,2]
* Output: false
*
*
* Constraints:
*
* * The number of nodes in each tree will be in the range `[1, 200]`.
* * Both of the given trees will have values in the range `[0, 200]`.
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
    pub fn leaf_similar(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        fn get_leafs(node: &Option<Rc<RefCell<TreeNode>>>, set: &mut Vec<i32>) {
            if let Some(v) = node {
                let v = v.borrow();
                if v.left.is_none() && v.right.is_none() {
                    set.push(v.val);
                } else {
                    get_leafs(&v.left, set);
                    get_leafs(&v.right, set);
                }
            }
        }

        let (mut leafs1, mut leafs2) = (Vec::<i32>::new(), Vec::<i32>::new());
        get_leafs(&root1, &mut leafs1);
        get_leafs(&root2, &mut leafs2);
        leafs1 == leafs2
    }
}
// @leetup=code

// @leetup=inject:after_code
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        assert!(Solution::leaf_similar(
            tree![3, 5, 1, 6, 2, 9, 8, null, null, 7, 4],
            tree![3, 5, 1, 6, 7, 4, 2, null, null, null, null, null, null, 9, 8]
        ))
    }

    #[test]
    fn example_2() {
        assert!(!Solution::leaf_similar(tree![1, 2, 3], tree![1, 3, 2]))
    }
}
// @leetup=inject:after_code
