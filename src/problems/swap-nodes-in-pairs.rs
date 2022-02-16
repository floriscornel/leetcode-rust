// @leetup=custom
// @leetup=info id=24 lang=rust slug=swap-nodes-in-pairs

/*
* Given a linked list, swap every two adjacent nodes and return its head. You must
* solve the problem without modifying the values in the list's nodes (i.e., only
* nodes themselves may be changed.)
*
*
* Example 1:
*
* []
* Input: head = [1,2,3,4]
* Output: [2,1,4,3]
*
* Example 2:
*
* Input: head = []
* Output: []
*
* Example 3:
*
* Input: head = [1]
* Output: [1]
*
*
* Constraints:
*
* * The number of nodes in the list is in the range `[0, 100]`.
* * `0 <= Node.val <= 100`
*
*/
// @leetup=custom
// @leetup=inject:before_code_ex
// before_code_exclude
#![allow(dead_code, unused_variables)]
struct Solution {}
use crate::util::linked_list::ListNode;
// @leetup=inject:before_code_ex

// @leetup=code

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }

impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() || head.as_ref().unwrap().next.is_none() {
            return head;
        }
        let reference = head;
        if let Some(mut first) = reference {
            if let Some(mut second) = first.next.take() {
                first.next = Solution::swap_pairs(second.next.take());
                second.next = Some(first);
                return Some(second);
            }
        }
        panic!("unreachable code.")
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
            Solution::swap_pairs(linked![1, 2, 3, 4]),
            linked![2, 1, 4, 3]
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::swap_pairs(linked![]), linked![]);
    }

    #[test]
    fn example_3() {
        assert_eq!(Solution::swap_pairs(linked![1]), linked![1]);
    }
}
// @leetup=inject:after_code
