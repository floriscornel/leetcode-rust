#![allow(dead_code, unused_variables)]
struct Solution {}
/** 104. Maximum Depth of Binary Tree - https://leetcode.com/problems/maximum-depth-of-binary-tree/ */
use crate::util::tree::TreeNode;
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

#[cfg(test)]
mod tests {
    use crate::util::tree::to_tree;

    #[test]
    fn example_1() {
        let input = tree![3, 9, 20, null, null, 15, 7];
        let output = 3;
        assert_eq!(super::Solution::max_depth(input), output);
    }

    #[test]
    fn example_2() {
        let input = tree![1, null, 2];
        let output = 2;
        assert_eq!(super::Solution::max_depth(input), output);
    }
}
