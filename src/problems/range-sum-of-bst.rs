// @leetup=custom
// @leetup=info id=938 lang=rust slug=range-sum-of-bst

/*
* Given the `root` node of a binary search tree and two integers `low` and `high`,
* return *the sum of values of all nodes with a value in the inclusive range
* *`[low, high]`.
*
*
* Example 1:
*
* []
* Input: root = [10,5,15,3,7,null,18], low = 7, high = 15
* Output: 32
* Explanation: Nodes 7, 10, and 15 are in the range [7, 15]. 7 + 10 + 15 = 32.
*
* Example 2:
*
* []
* Input: root = [10,5,15,3,7,13,18,1,null,6], low = 6, high = 10
* Output: 23
* Explanation: Nodes 6, 7, and 10 are in the range [6, 10]. 6 + 7 + 10 = 23.
*
*
* Constraints:
*
* * The number of nodes in the tree is in the range `[1, 2 * 104]`.
* * `1 <= Node.val <= 105`
* * `1 <= low <= high <= 105`
* * All `Node.val` are unique.
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
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        let mut sum = 0;
        if let Some(node) = root {
            let node = node.borrow();
            if node.val >= low && node.val <= high {
                sum += node.val;
            }
            if node.val > low {
                sum += Self::range_sum_bst(node.left.clone(), low, high);
            }
            if node.val < high {
                sum += Self::range_sum_bst(node.right.clone(), low, high);
            }
        }
        sum
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
            Solution::range_sum_bst(tree![10, 5, 15, 3, 7, null, 18], 7, 15),
            32
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::range_sum_bst(tree![10, 5, 15, 3, 7, 13, 18, 1, null, 6], 6, 10),
            23
        );
    }
}
// @leetup=inject:after_code
