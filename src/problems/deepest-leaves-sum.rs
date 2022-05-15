// @leetup=custom
// @leetup=info id=1302 lang=rust slug=deepest-leaves-sum

/*
* Given the `root` of a binary tree, return *the sum of values of its deepest
* leaves*.
*
*
* Example 1:
*
* []
* Input: root = [1,2,3,4,5,null,6,7,null,null,null,null,8]
* Output: 15
*
* Example 2:
*
* Input: root = [6,7,8,2,7,1,3,9,null,1,4,null,null,null,5]
* Output: 19
*
*
* Constraints:
*
* * The number of nodes in the tree is in the range `[1, 104]`.
* * `1 <= Node.val <= 100`
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
use std::cmp::Ordering;
use std::rc::Rc;

struct Leaf {
    val: i32,
    depth: i32,
}

impl Solution {
    pub fn deepest_leaves_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn helper(root: Option<&Rc<RefCell<TreeNode>>>, depth: i32) -> Option<Leaf> {
            match root {
                Some(node) => {
                    let borrowed = node.borrow();
                    let left_option = helper(borrowed.left.as_ref(), depth + 1);
                    let right_option = helper(borrowed.right.as_ref(), depth + 1);

                    if left_option.is_some() && right_option.is_none() {
                        left_option
                    } else if left_option.is_none() && right_option.is_some() {
                        right_option
                    } else if let (Some(left), Some(right)) = (left_option, right_option) {
                        match left.depth.cmp(&right.depth) {
                            Ordering::Equal => Some(Leaf {
                                val: left.val + right.val,
                                depth: left.depth,
                            }),
                            Ordering::Greater => Some(left),
                            Ordering::Less => Some(right),
                        }
                    } else {
                        Some(Leaf {
                            val: borrowed.val,
                            depth,
                        })
                    }
                }
                None => None,
            }
        }
        helper(root.as_ref(), 0).unwrap().val
    }
}
// @leetup=code

// @leetup=inject:after_code
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let root = tree![1, 2, 3, 4, 5, null, 6, 7, null, null, null, null, 8];
        assert_eq!(Solution::deepest_leaves_sum(root), 15);
    }

    #[test]
    fn example_2() {
        let root = tree![6, 7, 8, 2, 7, 1, 3, 9, null, 1, 4, null, null, null, 5];
        assert_eq!(Solution::deepest_leaves_sum(root), 19);
    }
}
// @leetup=inject:after_code
