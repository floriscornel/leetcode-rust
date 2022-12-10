// @leetup=custom
// @leetup=info id=1339 lang=rust slug=maximum-product-of-splitted-binary-tree

/*
* Given the `root` of a binary tree, split the binary tree into two subtrees by
* removing one edge such that the product of the sums of the subtrees is
* maximized.
*
* Return *the maximum product of the sums of the two subtrees*. Since the answer
* may be too large, return it modulo `109 + 7`.
*
* Note that you need to maximize the answer before taking the mod and not
* after taking it.
*
*
* Example 1:
*
* []
* Input: root = [1,2,3,4,5,6]
* Output: 110
* Explanation: Remove the red edge and get 2 binary trees with sum 11 and 10.
* Their product is 110 (11*10)
*
* Example 2:
*
* []
* Input: root = [1,null,2,3,4,null,null,5,6]
* Output: 90
* Explanation: Remove the red edge and get 2 binary trees with sum 15 and 6.Th
* eir product is 90 (15*6)
*
*
* Constraints:
*
* * The number of nodes in the tree is in the range `[2, 5 * 104]`.
* * `1 <= Node.val <= 104`
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
    pub fn max_product(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn sum(root: &Option<Rc<RefCell<TreeNode>>>, sums: &mut Vec<i64>) -> i64 {
            if let Some(node) = root {
                let node = node.borrow();
                let sum = node.val as i64 + sum(&node.left, sums) + sum(&node.right, sums);
                sums.push(sum);
                sum
            } else {
                0
            }
        }
        let mut sums = vec![];
        let total = sum(&root, &mut sums);
        let mut max = 0;
        for sum in sums {
            let product = sum * (total - sum);
            if product > max {
                max = product;
            }
        }
        (max % 1_000_000_007) as i32
    }
}
// @leetup=code

// @leetup=inject:after_code
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        assert_eq!(Solution::max_product(tree![1, 2, 3, 4, 5, 6]), 110);
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::max_product(tree![1, null, 2, 3, 4, null, null, 5, 6]),
            90
        );
    }
}
// @leetup=inject:after_code
